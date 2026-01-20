export type GlobalNotificationEvent = {
    projectId: string;
    level: "info" | "warn" | "error" | "success";
    message: string;
    summary: string
}