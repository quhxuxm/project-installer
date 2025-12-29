class ToolConfig {
    private _githubUsername: string;
    private _githubToken: string;
    private _projects: Map<string, ProjectConfig>;
    constructor(githubUsername: string, githubToken: string) {
        this._githubUsername = githubUsername;
        this._githubToken = githubToken;
        this._projects = new Map<string, ProjectConfig>();
    }

    get githubUsername(): string {
        return this._githubUsername;
    }

    get githubToken(): string {
        return this._githubToken;
    }

    set githubUsername(value: string) {
        this._githubUsername = value;
    }

    set githubToken(value: string) {
        this._githubToken = value;
    }

    get projects(): Map<string, ProjectConfig> {
        return this._projects;
    }

    set projects(value: Map<string, ProjectConfig>) {
        this._projects = value;
    }

    public addProject(project: ProjectConfig):void {
        this._projects.set(project.projectId, project);
    }

    public getProject(projectId: string): ProjectConfig|undefined{
        return this._projects.get(projectId);
    }

    public getAllProjects(): ProjectConfig[] {
        return Array.from(this._projects.values());
    }
}