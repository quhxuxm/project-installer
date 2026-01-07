import {CommandStateOutput} from "./command_state_output.ts";

export type ProjectStateOutput = {
    startupDependencies: string[];
    name: string | null;
    description: string | null;
    buildCommand: CommandStateOutput | null;
    runCommand: CommandStateOutput | null;
    debugCommand: CommandStateOutput | null;
    githubRepoUrl: string | null;
    githubBranches: string[];
    localRepoPath: string | null;
    configuredGithubBranch: string | null;
    backendProcessId: number | null;
}