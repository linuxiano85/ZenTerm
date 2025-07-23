import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { TerminalSession } from "./types";
import TerminalView from "./components/TerminalView";
import TabBar from "./components/TabBar";
import "./App.css";

function App() {
  const [sessions, setSessions] = useState<TerminalSession[]>([]);
  const [activeSessionId, setActiveSessionId] = useState<string | null>(null);
  const [isLoading, setIsLoading] = useState(true);

  // Initialize the app
  useEffect(() => {
    initializeApp();
  }, []);

  const initializeApp = async () => {
    try {
      // Create initial session
      await createNewSession();
      setIsLoading(false);
    } catch (error) {
      console.error("Failed to initialize app:", error);
      setIsLoading(false);
    }
  };

  const createNewSession = async () => {
    try {
      const sessionName = `Terminal ${sessions.length + 1}`;
      const sessionId = await invoke<string>("create_terminal_session", { 
        name: sessionName 
      });
      
      const newSession: TerminalSession = {
        id: sessionId,
        name: sessionName,
        active: true,
        current_directory: "~",
        commands: [],
      };
      
      setSessions(prev => [...prev, newSession]);
      setActiveSessionId(sessionId);
      
      // Set as active session in backend
      await invoke("set_active_session", { sessionId });
    } catch (error) {
      console.error("Failed to create session:", error);
    }
  };

  const closeSession = async (sessionId: string) => {
    try {
      await invoke("close_session", { sessionId });
      
      setSessions(prev => {
        const newSessions = prev.filter(s => s.id !== sessionId);
        
        // If we closed the active session, switch to another one
        if (sessionId === activeSessionId) {
          const nextSession = newSessions[0];
          if (nextSession) {
            setActiveSessionId(nextSession.id);
            invoke("set_active_session", { sessionId: nextSession.id });
          } else {
            setActiveSessionId(null);
          }
        }
        
        return newSessions;
      });
      
      // If no sessions left, create a new one
      if (sessions.length === 1) {
        await createNewSession();
      }
    } catch (error) {
      console.error("Failed to close session:", error);
    }
  };

  const switchSession = async (sessionId: string) => {
    try {
      setActiveSessionId(sessionId);
      await invoke("set_active_session", { sessionId });
    } catch (error) {
      console.error("Failed to switch session:", error);
    }
  };

  const handleCommand = (command: string) => {
    console.log("Command executed:", command);
    // Command execution is handled by the TerminalView component
  };

  const activeSession = sessions.find(s => s.id === activeSessionId);

  if (isLoading) {
    return (
      <div style={{
        display: 'flex',
        justifyContent: 'center',
        alignItems: 'center',
        height: '100vh',
        backgroundColor: '#1a1b26',
        color: '#c0caf5',
        fontFamily: 'JetBrains Mono, Consolas, "Courier New", monospace',
      }}>
        Initializing ZenTerm...
      </div>
    );
  }

  return (
    <div style={{
      height: '100vh',
      display: 'flex',
      flexDirection: 'column',
      backgroundColor: '#1a1b26',
      overflow: 'hidden',
    }}>
      <TabBar
        sessions={sessions}
        activeSessionId={activeSessionId}
        onSessionChange={switchSession}
        onSessionClose={closeSession}
        onNewSession={createNewSession}
      />
      
      <div style={{ flex: 1, position: 'relative' }}>
        {activeSession && (
          <TerminalView
            key={activeSession.id}
            session={activeSession}
            onCommand={handleCommand}
          />
        )}
      </div>
    </div>
  );
}

export default App;
