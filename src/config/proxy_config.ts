export class ProxyConfig {
    constructor(url: string) {
        this._url = url;
    }

    private _url: string;

    public get url(): string {
        return this._url;
    }

    public set url(value: string) {
        this._url = value;
    }
}