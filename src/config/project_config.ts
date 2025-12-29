class ProjectConfig{
    private _projectId: string;
    private _projectDescription: string | null;
    private _githubRepoUrl: string;
    private _githubBranch: string;
    private _buildCommand: CommandInfo | null;
    private _runCommand: CommandInfo | null;
    private _debugCommand: CommandInfo | null;
    private _projectLocalPath: string;

    constructor(projectId: string, githubRepoUrl: string, githubBranch: string,projectLocalPath: string) {
        this._projectId = projectId;
        this._projectDescription = null;
        this._githubRepoUrl = githubRepoUrl;
        this._githubBranch = githubBranch;
        this._buildCommand = null;
        this._runCommand = null;
        this._debugCommand=null;
        this._projectLocalPath = projectLocalPath;
    }

    get projectId(): string {
        return this._projectId;
    }

    set projectId(value: string) {
        this._projectId = value;
    }

    get projectDescription(): string | null {
        return this._projectDescription;
    }

    set projectDescription(value: string) {
        this._projectDescription = value;
    }

    get githubRepoUrl(): string {
        return this._githubRepoUrl;
    }

    set githubRepoUrl(value: string) {
        this._githubRepoUrl = value;
    }

    get githubBranch(): string {
        return this._githubBranch;
    }

    set githubBranch(value: string) {
        this._githubBranch = value;
    }

    get buildCommand(): CommandInfo | null {
        return this._buildCommand;
    }

    set buildCommand(value: CommandInfo) {
        this._buildCommand = value;
    }

    get runCommand(): CommandInfo | null {
        return this._runCommand;
    }

    set runCommand(value: CommandInfo) {
        this._runCommand = value;
    }

    get projectLocalPath(): string {
        return this._projectLocalPath;
    }

    set projectLocalPath(value: string) {
        this._projectLocalPath = value;
    }

    get debugCommand(): CommandInfo | null {
        return this._debugCommand;
    }

    set debugCommand(value: CommandInfo) {
        this._debugCommand = value;
    }
}