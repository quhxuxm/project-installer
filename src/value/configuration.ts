export class ToolConfiguration{
    private _github: GitHubConfiguration;
    private _projects: Map<string, ProjectConfiguration>;

    constructor() {
        this._projects = new Map<string, ProjectConfiguration>();
        this._github = new GitHubConfiguration();
    }

    get projects(): Map<string, ProjectConfiguration>{
        return this._projects;
    }

    set projects(value: Map<string, ProjectConfiguration>) {
        this._projects = value;
    }

    get github(): GitHubConfiguration {
        return this._github;
    }

    set github(value: GitHubConfiguration) {
        this._github = value;
    }
}

export class GitHubConfiguration{
    private _token:string | null;
    private _username:string | null;

    constructor() {
        this._token=null;
        this._username=null;
    }

    get token(): string | null{
        return this._token;
    }

    set token(value: string | null){
        this._token = value;
    }

    get username(): string | null {
        return this._username;
    }

    set username(value: string | null) {
        this._username = value;
    }
}
export class CommandConfiguration{
    private _cmd:string;
    private _args:string[];
    public constructor(cmd:string) {
        this._cmd=cmd;
        this._args = [];
    }

    get cmd(): string {
        return this._cmd;
    }

    set cmd(value: string) {
        this._cmd = value;
    }

    get args(): string[] {
        return this._args;
    }

    set args(value: string[]) {
        this._args = value;
    }
}

export class ProjectConfiguration{
    private _id:string;
    private _description:string | null;
    private _buildCommand:CommandConfiguration | null;
    private _runCommand:CommandConfiguration | null;
    private _debugCommand:CommandConfiguration | null;
    private _githubRepoUrl:string;
    private _localRepoPath:string;
    private _githubBranch:string;
    private readonly _dependencies:ProjectConfiguration[];


    constructor(id: string, githubRepoUrl: string, localRepoPath: string, githubBranch: string) {
        this._id = id;
        this._githubRepoUrl = githubRepoUrl;
        this._localRepoPath = localRepoPath;
        this._githubBranch = githubBranch;
        this._buildCommand=null;
        this._debugCommand=null;
        this._runCommand=null;
        this._description=null;
        this._dependencies = [];
    }

    get id(): string {
        return this._id;
    }

    set id(value: string) {
        this._id = value;
    }

    get description(): string | null {
        return this._description;
    }

    set description(value: string) {
        this._description = value;
    }

    get buildCommand(): CommandConfiguration | null {
        return this._buildCommand;
    }

    set buildCommand(value: CommandConfiguration | null) {
        this._buildCommand = value;
    }

    get runCommand(): CommandConfiguration | null {
        return this._runCommand;
    }

    set runCommand(value: CommandConfiguration | null) {
        this._runCommand = value;
    }

    get debugCommand(): CommandConfiguration | null {
        return this._debugCommand;
    }

    set debugCommand(value: CommandConfiguration | null) {
        this._debugCommand = value;
    }

    get githubRepoUrl(): string {
        return this._githubRepoUrl;
    }

    set githubRepoUrl(value: string) {
        this._githubRepoUrl = value;
    }

    get localRepoPath(): string {
        return this._localRepoPath;
    }

    set localRepoPath(value: string) {
        this._localRepoPath = value;
    }

    get githubBranch(): string {
        return this._githubBranch;
    }

    set githubBranch(value: string) {
        this._githubBranch = value;
    }

    get dependencies(): ProjectConfiguration[] {
        return this._dependencies;
    }

    public addDependency(dependency: ProjectConfiguration):void {
        this._dependencies.push(dependency);
    }
}