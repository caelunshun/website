export interface Output {
    timeout: number;
    message: () => string;
    newline: boolean;
    typewriter?: boolean;
    noslow?: boolean;
    caret?: boolean;
}

export interface TerminalOutput {
    message: string;
    caret: boolean;
}