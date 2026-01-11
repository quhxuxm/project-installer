export type CommandRuntimeState = {
    command: string;
    args: string[];
}

export type ProjectRuntimeSummary = {
    projectId: string;
    name: string;
    description: string;
};

export type ProjectRuntimeDetail = {
    name: string;
    description: string;
    githubBranch: string;
    githubRepoUrl: string;
    localRepoPath: string;
    currentProcess: string | null | undefined;
    availableGithubBranches: string[];
    buildCommand: CommandRuntimeState | null | undefined;
    runCommand: CommandRuntimeState | null | undefined;
    stopCommand: CommandRuntimeState | null | undefined;
    debugCommand: CommandRuntimeState | null | undefined;
};

export type ProjectRuntimeUpdate = {
    projectId: string;
    githubBranch: string;
    githubRepoUrl: string;
    localRepoPath: string;
    buildCommand: string | null | undefined;
    runCommand: string | null | undefined;
    debugCommand: string | null | undefined;
}
