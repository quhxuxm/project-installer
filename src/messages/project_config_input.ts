export type ProjectConfigInput = {
    projectId: string,
    selectedGithubBranch: string,
    githubRepoUrl: string,
    localRepoPath: string,
    buildCommand: string,
    runCommand: string,
    debugCommand: string,
}