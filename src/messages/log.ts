export enum LogLevel {
    DEBUG = 'debug',
    INFO = 'info',
    WARN = 'warn',
    ERROR = 'error',
}

export enum LogSource {
    GITHUB = 'github',
    BACKEND_COMMAND = 'build-command',
}

export type LogEvent = {
    projectId: string;
    level: LogLevel;
    message: string;
}