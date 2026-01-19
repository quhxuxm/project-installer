export type ProjectRuntimeSummary = {
    projectId: string;
    name: string;
    description: string;
};

export type PropertyItem = {
    key: string;
    value: string
}


export type RunningCommandStatus = {
    commandType: "build" | "run" | "debug" | "save" | "fetchCode";
    projectId: string;
    status: "running" | "terminatedSuccess" | "terminatedFailure";
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
    customizedProperties: PropertyItem[];
};

export type ProjectRuntimeUpdate = {
    projectId: string;
    workingBranch: string;
    remoteRepoUrl: string;
    localRepoPath: string;
    buildCommand: string | null | undefined;
    runCommand: string | null | undefined;
    debugCommand: string | null | undefined;
    customizedProperties: PropertyItem[];
}
