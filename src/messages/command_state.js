export class CommandState {
     constructor() {
        this._cmd = null;
        this._args = [];
    }


    get cmd() {
        return this._cmd;
    }

    set cmd(value) {
        this._cmd = value;
    }

    get args() {
        return this._args;
    }

    set args(value) {
        this._args = value;
    }
}