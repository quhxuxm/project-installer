import {CommandState} from "./command_state.ts";

export type ProjectState = {
    startupDependencies: string[];
    name: string | null;
    description: string | null;
    buildCommand: CommandState | null;
    runCommand: CommandState | null;
    debugCommand: CommandState | null;
    githubRepoUrl: string | null;
    localRepoPath: string | null;
    configuredGithubBranch: string | null;
    backendProcessId: number | null;
}