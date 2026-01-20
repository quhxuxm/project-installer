export type GlobalLogEvent = {
    projectId: string;
    level: "error" | "info" | "debug" | "warn";
    message: string;
}