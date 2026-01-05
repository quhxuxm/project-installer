

export class ProjectState {
    constructor() {
        this._name = null;
        this._githubRepoUrl = null;
        this._localRepoPath = null;
        this._configuredGithubBranch = null;
        this._buildCommand = null;
        this._debugCommand = null;
        this._runCommand = null;
        this._description = null;
        this._startupDependencies = [];
        this._backendProcessId = null;
    }


    get name() {
        return this._name;
    }

    set name(value) {
        this._name = value;
    }

    get githubRepoUrl() {
        return this._githubRepoUrl;
    }

    set githubRepoUrl(value) {
        this._githubRepoUrl = value;
    }

    get localRepoPath() {
        return this._localRepoPath;
    }

    set localRepoPath(value) {
        this._localRepoPath = value;
    }

    get configuredGithubBranch() {
        return this._configuredGithubBranch;
    }

    set configuredGithubBranch(value) {
        this._configuredGithubBranch = value;
    }

    get buildCommand() {
        return this._buildCommand;
    }

    set buildCommand(value) {
        this._buildCommand = value;
    }

    get debugCommand() {
        return this._debugCommand;
    }

    set debugCommand(value) {
        this._debugCommand = value;
    }

    get runCommand() {
        return this._runCommand;
    }

    set runCommand(value) {
        this._runCommand = value;
    }

    get description() {
        return this._description;
    }

    set description(value) {
        this._description = value;
    }

    get startupDependencies() {
        return this._startupDependencies;
    }

    set startupDependencies(value) {
        this._startupDependencies = value;
    }

    get backendProcessId() {
        return this._backendProcessId;
    }

    set backendProcessId(value) {
        this._backendProcessId = value;
    }
}