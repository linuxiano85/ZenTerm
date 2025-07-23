use anyhow::Result;
use portable_pty::{CommandBuilder, PtySize};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::{BufRead, BufReader, Write};
use std::sync::{Arc, Mutex};
use std::thread;
use tokio::sync::broadcast;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerminalOutput {
    pub session_id: String,
    pub content: String,
    pub is_error: bool,
    pub timestamp: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandBlock {
    pub id: String,
    pub session_id: String,
    pub command: String,
    pub output: Vec<TerminalOutput>,
    pub exit_code: Option<i32>,
    pub start_time: u64,
    pub end_time: Option<u64>,
    pub status: CommandStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CommandStatus {
    Running,
    Completed,
    Failed,
}

pub struct PtyManager {
    sessions: Arc<Mutex<HashMap<String, PtySession>>>,
    output_sender: broadcast::Sender<TerminalOutput>,
}

pub struct PtySession {
    pub id: String,
    pub pty: Box<dyn portable_pty::MasterPty + Send>,
    pub writer: Box<dyn Write + Send>,
    pub current_command: Option<CommandBlock>,
}

impl PtyManager {
    pub fn new() -> Self {
        let (output_sender, _) = broadcast::channel(1000);
        
        PtyManager {
            sessions: Arc::new(Mutex::new(HashMap::new())),
            output_sender,
        }
    }

    pub fn create_session(&self) -> Result<String> {
        let session_id = Uuid::new_v4().to_string();
        
        let pty_system = portable_pty::native_pty_system();
        let pty_pair = pty_system.openpty(PtySize {
            rows: 24,
            cols: 80,
            pixel_width: 0,
            pixel_height: 0,
        })?;

        let mut cmd = CommandBuilder::new("/bin/bash");
        cmd.env("TERM", "xterm-256color");
        
        let child = pty_pair.slave.spawn_command(cmd)?;
        std::mem::drop(child); // The child process will continue running

        let reader = pty_pair.master.try_clone_reader()?;
        let writer = pty_pair.master.take_writer()?;

        let session = PtySession {
            id: session_id.clone(),
            pty: pty_pair.master,
            writer,
            current_command: None,
        };

        // Start reading output in a separate thread
        let sessions_arc = Arc::clone(&self.sessions);
        let output_sender = self.output_sender.clone();
        let session_id_clone = session_id.clone();

        thread::spawn(move || {
            let buf_reader = BufReader::new(reader);
            for line in buf_reader.lines() {
                if let Ok(content) = line {
                    let output = TerminalOutput {
                        session_id: session_id_clone.clone(),
                        content,
                        is_error: false,
                        timestamp: std::time::SystemTime::now()
                            .duration_since(std::time::UNIX_EPOCH)
                            .unwrap()
                            .as_millis() as u64,
                    };
                    
                    let _ = output_sender.send(output);
                }
            }
        });

        self.sessions.lock().unwrap().insert(session_id.clone(), session);
        Ok(session_id)
    }

    pub fn write_command(&self, session_id: &str, command: &str) -> Result<String> {
        let mut sessions = self.sessions.lock().unwrap();
        
        if let Some(session) = sessions.get_mut(session_id) {
            let command_id = Uuid::new_v4().to_string();
            let timestamp = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64;

            let command_block = CommandBlock {
                id: command_id.clone(),
                session_id: session_id.to_string(),
                command: command.to_string(),
                output: Vec::new(),
                exit_code: None,
                start_time: timestamp,
                end_time: None,
                status: CommandStatus::Running,
            };

            session.current_command = Some(command_block);
            
            // Write command to PTY
            writeln!(session.writer, "{}", command)?;
            
            Ok(command_id)
        } else {
            Err(anyhow::anyhow!("Session not found"))
        }
    }

    pub fn get_output_receiver(&self) -> broadcast::Receiver<TerminalOutput> {
        self.output_sender.subscribe()
    }

    pub fn resize_session(&self, session_id: &str, rows: u16, cols: u16) -> Result<()> {
        let sessions = self.sessions.lock().unwrap();
        
        if let Some(session) = sessions.get(session_id) {
            session.pty.resize(PtySize {
                rows,
                cols,
                pixel_width: 0,
                pixel_height: 0,
            })?;
        }
        
        Ok(())
    }

    pub fn close_session(&self, session_id: &str) -> Result<()> {
        let mut sessions = self.sessions.lock().unwrap();
        sessions.remove(session_id);
        Ok(())
    }
}