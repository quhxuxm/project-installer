import {GitHubStateOutput} from "./github_state_output.ts";
import {ProjectStateOutput} from "./project_state_output.ts";

export type ApplicationStateOutput = {
    github: GitHubStateOutput | null;
    projects: {
        [key: string]: ProjectStateOutput;
    };
}