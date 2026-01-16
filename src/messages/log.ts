export enum GlobalLogLevel {
    DEBUG = 'debug',
    INFO = 'info',
    WARN = 'warn',
    ERROR = 'error',
}


export type GlobalLogEvent = {
    projectId: string;
    level: GlobalLogLevel;
    message: string;
}