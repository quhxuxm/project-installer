export enum GlobalNotificationLevel {
    INFO = 'info',
    WARN = 'warn',
    ERROR = 'error',
    Success = 'success',
}


export type GlobalNotificationEvent = {
    projectId: string;
    level: GlobalNotificationLevel;
    message: string;
    summary: string
}