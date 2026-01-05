export class CommandState {
    public cmd: string | null;
    public args: string[];

    public constructor() {
        this.cmd = null;
        this.args = [];
    }
}
