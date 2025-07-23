import React, { useEffect, useRef, useState } from 'react';
import { Terminal } from '@xterm/xterm';
import { FitAddon } from '@xterm/addon-fit';
import { WebLinksAddon } from '@xterm/addon-web-links';
import { invoke } from '@tauri-apps/api/core';
import '@xterm/xterm/css/xterm.css';
import { TerminalSession } from '../types';

interface TerminalViewProps {
  session: TerminalSession;
  onCommand?: (command: string) => void;
}

export const TerminalView: React.FC<TerminalViewProps> = ({ session, onCommand }) => {
  const terminalRef = useRef<any>(null);
  const terminal = useRef<Terminal | null>(null);
  const fitAddon = useRef<FitAddon | null>(null);
  const [isInitialized, setIsInitialized] = useState(false);

  useEffect(() => {
    if (!terminalRef.current || isInitialized) return;

    // Initialize terminal
    const term = new Terminal({
      cursorBlink: true,
      fontSize: 14,
      fontFamily: 'JetBrains Mono, Consolas, "Courier New", monospace',
      theme: {
        background: '#1a1b26',
        foreground: '#c0caf5',
        cursor: '#c0caf5',
        selectionBackground: '#283457',
        black: '#15161e',
        red: '#f7768e',
        green: '#9ece6a',
        yellow: '#e0af68',
        blue: '#7aa2f7',
        magenta: '#bb9af7',
        cyan: '#7dcfff',
        white: '#a9b1d6',
        brightBlack: '#414868',
        brightRed: '#f7768e',
        brightGreen: '#9ece6a',
        brightYellow: '#e0af68',
        brightBlue: '#7aa2f7',
        brightMagenta: '#bb9af7',
        brightCyan: '#7dcfff',
        brightWhite: '#c0caf5',
      },
    });

    // Add addons
    const fit = new FitAddon();
    const webLinks = new WebLinksAddon();
    
    term.loadAddon(fit);
    term.loadAddon(webLinks);
    
    // Open terminal
    term.open(terminalRef.current);
    fit.fit();

    // Store references
    terminal.current = term;
    fitAddon.current = fit;
    
    // Handle data input
    let currentLine = '';
    term.onData((data) => {
      // Handle enter key
      if (data === '\r') {
        term.write('\r\n');
        if (currentLine.trim()) {
          onCommand?.(currentLine.trim());
          executeCommand(currentLine.trim());
        }
        currentLine = '';
        return;
      }
      
      // Handle backspace
      if (data === '\x7F') {
        if (currentLine.length > 0) {
          currentLine = currentLine.slice(0, -1);
          term.write('\b \b');
        }
        return;
      }
      
      // Handle regular characters
      if (data.charCodeAt(0) >= 32) {
        currentLine += data;
        term.write(data);
      }
    });

    // Show initial prompt
    term.write('$ ');
    
    setIsInitialized(true);

    // Handle resize
    const handleResize = () => {
      if (fitAddon.current) {
        fitAddon.current.fit();
        // Update backend with new size
        invoke('resize_terminal', {
          sessionId: session.id,
          rows: terminal.current?.rows || 24,
          cols: terminal.current?.cols || 80,
        }).catch(console.error);
      }
    };

    window.addEventListener('resize', handleResize);

    return () => {
      window.removeEventListener('resize', handleResize);
      term.dispose();
    };
  }, [isInitialized, session.id, onCommand]);

  const executeCommand = async (command: string) => {
    if (!terminal.current) return;

    try {
      await invoke('execute_command', {
        sessionId: session.id,
        command,
      });
      
      // The output will be handled by the output listener in the parent component
    } catch (error) {
      console.error('Failed to execute command:', error);
      terminal.current.write(`\r\nError: ${error}\r\n$ `);
    }
  };

  const writeOutput = (output: string) => {
    if (terminal.current) {
      terminal.current.write(output);
    }
  };

  // Expose method to parent component
  React.useImperativeHandle(terminalRef, () => ({
    writeOutput,
    fit: () => fitAddon.current?.fit(),
  }));

  return (
    <div 
      ref={terminalRef} 
      style={{ 
        width: '100%', 
        height: '100%',
        backgroundColor: '#1a1b26'
      }} 
    />
  );
};

export default TerminalView;