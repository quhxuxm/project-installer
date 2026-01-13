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
    level: LogLevel;
    date: Date;
    message: string;
    source: LogSource;
}