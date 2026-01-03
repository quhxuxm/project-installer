export class GlobalStatus {
    public github: GitHubStatus;
    public projects: Map<string, ProjectStatus>;

    constructor() {
        this.projects = new Map<string, ProjectStatus>();
        this.github = new GitHubStatus();
    }
}

export class GitHubStatus {
    public token: string | null;
    public username: string | null;

    constructor() {
        this.token = null;
        this.username = null;
    }
}

export class CommandStatus {
    public cmd: string;
    public args: string[];

    public constructor(cmd: string) {
        this.cmd = cmd;
        this.args = [];
    }
}

export class ProjectStatus {
    public startupDependencies: string[];
    public name: string | null;
    public description: string | null;
    public buildCommand: CommandStatus | null;
    public runCommand: CommandStatus | null;
    public debugCommand: CommandStatus | null;
    public githubRepoUrl: string | null;
    public localRepoPath: string | null;
    public configuredGithubBranch: string | null;
    public backendProcessId: number | null;

    constructor() {
        this.name = null;
        this.githubRepoUrl = null;
        this.localRepoPath = null;
        this.configuredGithubBranch = null;
        this.buildCommand = null;
        this.debugCommand = null;
        this.runCommand = null;
        this.description = null;
        this.startupDependencies = [];
        this.backendProcessId = null;
    }
}