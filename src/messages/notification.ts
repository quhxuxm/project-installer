export enum GlobalNotificationLevel {
    INFO = 'info',
    WARN = 'warn',
    ERROR = 'error',
}


export type GlobalNotificationEvent = {
    projectId: string;
    level: GlobalNotificationLevel;
    message: string;
    summary: string
}