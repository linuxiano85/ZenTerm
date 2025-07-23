use crate::terminal::{CommandBlock, PtyManager, TerminalOutput};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::sync::broadcast;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerminalSession {
    pub id: String,
    pub name: String,
    pub active: bool,
    pub current_directory: String,
    pub commands: Vec<CommandBlock>,
}

pub struct SessionManager {
    sessions: Arc<Mutex<HashMap<String, TerminalSession>>>,
    pty_manager: Arc<PtyManager>,
    active_session: Arc<Mutex<Option<String>>>,
}

impl SessionManager {
    pub fn new() -> Self {
        SessionManager {
            sessions: Arc::new(Mutex::new(HashMap::new())),
            pty_manager: Arc::new(PtyManager::new()),
            active_session: Arc::new(Mutex::new(None)),
        }
    }

    pub fn create_session(&self, name: String) -> Result<String> {
        let session_id = self.pty_manager.create_session()?;
        
        let session = TerminalSession {
            id: session_id.clone(),
            name,
            active: true,
            current_directory: std::env::var("HOME").unwrap_or_else(|_| "/".to_string()),
            commands: Vec::new(),
        };

        self.sessions.lock().unwrap().insert(session_id.clone(), session);
        *self.active_session.lock().unwrap() = Some(session_id.clone());

        Ok(session_id)
    }

    pub fn get_session(&self, session_id: &str) -> Option<TerminalSession> {
        self.sessions.lock().unwrap().get(session_id).cloned()
    }

    pub fn list_sessions(&self) -> Vec<TerminalSession> {
        self.sessions.lock().unwrap().values().cloned().collect()
    }

    pub fn set_active_session(&self, session_id: String) -> Result<()> {
        if self.sessions.lock().unwrap().contains_key(&session_id) {
            *self.active_session.lock().unwrap() = Some(session_id);
            Ok(())
        } else {
            Err(anyhow::anyhow!("Session not found"))
        }
    }

    pub fn get_active_session(&self) -> Option<String> {
        self.active_session.lock().unwrap().clone()
    }

    pub fn execute_command(&self, session_id: &str, command: &str) -> Result<String> {
        self.pty_manager.write_command(session_id, command)
    }

    pub fn get_output_receiver(&self) -> broadcast::Receiver<TerminalOutput> {
        self.pty_manager.get_output_receiver()
    }

    pub fn resize_session(&self, session_id: &str, rows: u16, cols: u16) -> Result<()> {
        self.pty_manager.resize_session(session_id, rows, cols)
    }

    pub fn close_session(&self, session_id: &str) -> Result<()> {
        self.pty_manager.close_session(session_id)?;
        self.sessions.lock().unwrap().remove(session_id);
        
        // If this was the active session, find another one or clear it
        let active = self.active_session.lock().unwrap().clone();
        if active.as_ref() == Some(&session_id.to_string()) {
            let sessions = self.sessions.lock().unwrap();
            let new_active = sessions.keys().next().cloned();
            *self.active_session.lock().unwrap() = new_active;
        }
        
        Ok(())
    }
}