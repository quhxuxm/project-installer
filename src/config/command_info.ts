export class CommandInfo {
    constructor(cmd: string, args: string[]) {
        this._cmd = cmd;
        this._args = args;
    }

    private _cmd: string;

    get cmd(): string {
        return this._cmd;
    }

    set cmd(value: string) {
        this._cmd = value;
    }

    private _args: string[];

    get args(): string[] {
        return this._args;
    }

    set args(value: string[]) {
        this._args = value;
    }
}