import {GitHubState} from "./github_state.ts";
import {ProjectState} from "./project_state.ts";

export type ApplicationState = {
    github: GitHubState | null;
    projects: {
        [key: string]: ProjectState;
    };
}