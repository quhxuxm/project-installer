import {CommandState} from "./command_state.ts";

export class ProjectState {
    public startupDependencies: string[];
    public name: string | null;
    public description: string | null;
    public buildCommand: CommandState | null;
    public runCommand: CommandState | null;
    public debugCommand: CommandState | null;
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