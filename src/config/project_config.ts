import {CommandInfo} from "./command_info.ts";

export class ProjectConfig {
    constructor(projectId: string, githubRepoUrl: string, githubBranch: string, projectLocalPath: string) {
        this._projectId = projectId;
        this._description = null;
        this._githubRepoUrl = githubRepoUrl;
        this._githubBranch = githubBranch;
        this._buildCommand = null;
        this._runCommand = null;
        this._debugCommand = null;
        this._projectLocalPath = projectLocalPath;
    }

    private _projectId: string;

    get projectId(): string {
        return this._projectId;
    }

    set projectId(value: string) {
        this._projectId = value;
    }

    private _description: string | null;

    get description(): string | null {
        return this._description;
    }

    set description(value: string) {
        this._description = value;
    }

    private _githubRepoUrl: string;

    get githubRepoUrl(): string {
        return this._githubRepoUrl;
    }

    set githubRepoUrl(value: string) {
        this._githubRepoUrl = value;
    }

    private _githubBranch: string;

    get githubBranch(): string {
        return this._githubBranch;
    }

    set githubBranch(value: string) {
        this._githubBranch = value;
    }

    private _buildCommand: CommandInfo | null;

    get buildCommand(): CommandInfo | null {
        return this._buildCommand;
    }

    set buildCommand(value: CommandInfo) {
        this._buildCommand = value;
    }

    private _runCommand: CommandInfo | null;

    get runCommand(): CommandInfo | null {
        return this._runCommand;
    }

    set runCommand(value: CommandInfo) {
        this._runCommand = value;
    }

    private _debugCommand: CommandInfo | null;

    get debugCommand(): CommandInfo | null {
        return this._debugCommand;
    }

    set debugCommand(value: CommandInfo) {
        this._debugCommand = value;
    }

    private _projectLocalPath: string;

    get projectLocalPath(): string {
        return this._projectLocalPath;
    }

    set projectLocalPath(value: string) {
        this._projectLocalPath = value;
    }
}