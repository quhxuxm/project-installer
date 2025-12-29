class CommandInfo {
    private _cmd: string;
    private _args: string[];

    constructor(cmd: string, args: string[]) {
        this._cmd = cmd;
        this._args = args;
    }


    get cmd(): string {
        return this._cmd;
    }

    set cmd(value: string) {
        this._cmd = value;
    }

    get args(): string[] {
        return this._args;
    }

    set args(value: string[]) {
        this._args = value;
    }
}