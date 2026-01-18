export type ProjectRuntimeSummary = {
    projectId: string;
    name: string;
    description: string;
};

export type PropertyItem = {
    key: string;
    value: string
}


export type CurrentProcess = {
    processType: "build" | "run" | "debug" | "stop" | "gitClone";
    pid: number | null | undefined;
    projectId: string;
    status: "running" | "terminatedSuccess" | "terminatedFailure";
}

export type ProjectRuntimeDetail = {
    name: string;
    description: string;
    workingBranch: string;
    remoteRepoUrl: string;
    localRepoPath: string;
    currentProcess: CurrentProcess | null | undefined;
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
