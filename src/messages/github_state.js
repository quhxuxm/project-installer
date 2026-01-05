export class GitHubState {

    constructor() {
        this._token = null;
        this._username = null;
    }


    get token() {
        return this._token;
    }

    set token(value) {
        this._token = value;
    }

    get username() {
        return this._username;
    }

    set username(value) {
        this._username = value;
    }
}