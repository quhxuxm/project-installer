import {GitHubState} from "./github_state.js";

export class GlobalState {

    constructor() {
        this._projects = new Map();
        this._github = new GitHubState();
    }

    get projects() {
        return this._projects;
    }

    set projects(value) {
        this._projects = value;
    }

    get github() {
        return this._github;
    }

    set github(value) {
        this._github = value;
    }
}