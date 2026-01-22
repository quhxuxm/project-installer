export type ProjectRuntimeSummary = {
    projectId: string;
    name: string;
    description: string;
};


export type RunningCommandStatus = {
    commandType: "Build" | "Run" | "Debug" | "Save" | "FetchCode" | "Stop";
    projectId: string;
    status: "Running" | "TerminatedSuccess" | "TerminatedFailure";
}

export type ProjectRuntimeDetail = {
    name: string;
    description: string;
    workingBranch: string;
    remoteRepoUrl: string;
    localRepoPath: string;
    currentRunningCommandStatus: RunningCommandStatus | null | undefined;
    availableBranches: string[];
    buildCommand: string | null | undefined;
    runCommand: string | null | undefined;
    stopCommand: string | null | undefined;
    debugCommand: string | null | undefined;
    customizedBuildCommand: string | null | undefined;
    customizedRunCommand: string | null | undefined;
    customizedStopCommand: string | null | undefined;
    customizedDebugCommand: string | null | undefined;
    defaultCustomizedProperties: string | null | undefined;
    branchCustomizedProperties: string | null | undefined;
};

export type ProjectRuntimeUpdate = {
    projectId: string;
    workingBranch: string;
    remoteRepoUrl: string;
    localRepoPath: string;
    buildCommand: string | null | undefined;
    runCommand: string | null | undefined;
    debugCommand: string | null | undefined;
    customizedProperties: string | null | undefined;
}
