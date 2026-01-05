import {GitHubState} from "./github_state.ts";
import {ProjectState} from "./project_state.ts";

export class ApplicationState {
    public github: GitHubState | null;
    public projects: Map<string, ProjectState>;

    constructor() {
        this.projects = new Map<string, ProjectState>();
        this.github = null;
    }
}