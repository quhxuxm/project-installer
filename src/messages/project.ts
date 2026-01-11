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
    buildCommand: string | null | undefined;
    runCommand: string | null | undefined;
    stopCommand: string | null | undefined;
    debugCommand: string | null | undefined;
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
