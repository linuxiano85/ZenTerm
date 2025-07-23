export interface TerminalSession {
  id: string;
  name: string;
  active: boolean;
  current_directory: string;
  commands: CommandBlock[];
}

export interface CommandBlock {
  id: string;
  session_id: string;
  command: string;
  output: TerminalOutput[];
  exit_code?: number;
  start_time: number;
  end_time?: number;
  status: CommandStatus;
}

export interface TerminalOutput {
  session_id: string;
  content: string;
  is_error: boolean;
  timestamp: number;
}

export enum CommandStatus {
  Running = "Running",
  Completed = "Completed",
  Failed = "Failed",
}

export interface Theme {
  name: string;
  colors: {
    background: string;
    foreground: string;
    cursor: string;
    selection: string;
    black: string;
    red: string;
    green: string;
    yellow: string;
    blue: string;
    magenta: string;
    cyan: string;
    white: string;
    brightBlack: string;
    brightRed: string;
    brightGreen: string;
    brightYellow: string;
    brightBlue: string;
    brightMagenta: string;
    brightCyan: string;
    brightWhite: string;
  };
}