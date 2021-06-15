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

export interface DocResponse {
    html: string;
    topics: Topic[]
}

export interface Topic {
    name: string;
    hash: string;
}