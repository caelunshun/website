export interface Output {
    timeout: number;
    message: () => string;
    newline: boolean;
    typewriter?: boolean;
    noslow?: boolean;
}