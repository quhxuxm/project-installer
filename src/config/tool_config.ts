import {ProjectConfig} from "./project_config.ts";

export class ToolConfig {
    constructor(githubUsername: string, githubToken: string) {
        this._githubUsername = githubUsername;
        this._githubToken = githubToken;
        this._projects = new Map<string, ProjectConfig>();
    }

    private _githubUsername: string;

    get githubUsername(): string {
        return this._githubUsername;
    }

    set githubUsername(value: string) {
        this._githubUsername = value;
    }

    private _githubToken: string;

    get githubToken(): string {
        return this._githubToken;
    }

    set githubToken(value: string) {
        this._githubToken = value;
    }

    private _projects: Map<string, ProjectConfig>;

    get projects(): Map<string, ProjectConfig> {
        return this._projects;
    }

    set projects(value: Map<string, ProjectConfig>) {
        this._projects = value;
    }

    public addProject(project: ProjectConfig): void {
        this._projects.set(project.projectId, project);
    }

    public getProject(projectId: string): ProjectConfig | undefined {
        return this._projects.get(projectId);
    }

    public getAllProjects(): ProjectConfig[] {
        return Array.from(this._projects.values());
    }
}