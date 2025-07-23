import React from 'react';
import { X, Plus } from 'lucide-react';
import { TerminalSession } from '../types';

interface TabBarProps {
  sessions: TerminalSession[];
  activeSessionId: string | null;
  onSessionChange: (sessionId: string) => void;
  onSessionClose: (sessionId: string) => void;
  onNewSession: () => void;
}

export const TabBar: React.FC<TabBarProps> = ({
  sessions,
  activeSessionId,
  onSessionChange,
  onSessionClose,
  onNewSession,
}) => {
  return (
    <div style={{
      display: 'flex',
      alignItems: 'center',
      backgroundColor: '#16161e',
      borderBottom: '1px solid #283457',
      padding: '0 8px',
      minHeight: '40px',
    }}>
      {sessions.map((session) => (
        <div
          key={session.id}
          onClick={() => onSessionChange(session.id)}
          style={{
            display: 'flex',
            alignItems: 'center',
            padding: '6px 12px',
            backgroundColor: session.id === activeSessionId ? '#1a1b26' : 'transparent',
            border: session.id === activeSessionId ? '1px solid #283457' : '1px solid transparent',
            borderBottom: 'none',
            borderRadius: '6px 6px 0 0',
            marginRight: '4px',
            cursor: 'pointer',
            color: session.id === activeSessionId ? '#c0caf5' : '#737aa2',
            fontSize: '13px',
            maxWidth: '200px',
            position: 'relative',
          }}
        >
          <span style={{ 
            marginRight: '8px',
            overflow: 'hidden',
            textOverflow: 'ellipsis',
            whiteSpace: 'nowrap',
          }}>
            {session.name}
          </span>
          <button
            onClick={(e) => {
              e.stopPropagation();
              onSessionClose(session.id);
            }}
            style={{
              background: 'none',
              border: 'none',
              color: 'inherit',
              cursor: 'pointer',
              padding: '2px',
              display: 'flex',
              alignItems: 'center',
              borderRadius: '2px',
              opacity: 0.7,
            }}
            onMouseEnter={(e) => {
              e.currentTarget.style.opacity = '1';
              e.currentTarget.style.backgroundColor = '#283457';
            }}
            onMouseLeave={(e) => {
              e.currentTarget.style.opacity = '0.7';
              e.currentTarget.style.backgroundColor = 'transparent';
            }}
          >
            <X size={12} />
          </button>
        </div>
      ))}
      
      <button
        onClick={onNewSession}
        style={{
          background: 'none',
          border: '1px solid #283457',
          color: '#737aa2',
          cursor: 'pointer',
          padding: '6px 8px',
          display: 'flex',
          alignItems: 'center',
          borderRadius: '4px',
          marginLeft: '8px',
        }}
        onMouseEnter={(e) => {
          e.currentTarget.style.color = '#c0caf5';
          e.currentTarget.style.borderColor = '#414868';
        }}
        onMouseLeave={(e) => {
          e.currentTarget.style.color = '#737aa2';
          e.currentTarget.style.borderColor = '#283457';
        }}
      >
        <Plus size={14} />
      </button>
    </div>
  );
};

export default TabBar;