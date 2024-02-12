type PromiseApi = { [Key: string]: (...args: any) => Promise<any> };

type SimpleReceiver<T> = { target: string | undefined; listen: () => void };
type SimpleClient<T> = T;

export type BackendRequest = {
    method: keyof PromiseApi;
    args: any[];
};

export function createReceiver<A, T>(api: A & PromiseApi, target: string): SimpleReceiver<A> {
    return {
        target,
        listen() {
            chrome.runtime.onMessage.addListener((message: BackendRequest, _, respond) => {
                (api[message.method] as any)(...message.args).then(respond);
                return true;
            });
        },
    };
}

export function createClient<T extends SimpleReceiver<any>>(
    { target }: T,
    guard: () => Promise<void> = async () => {},
): SimpleClient<T extends SimpleReceiver<infer R> ? R : never> {
    return new Proxy({}, {
        get(_, method) {
            return async (...args: any[]) => {
                return new Promise(async (resolve) => {
                    await guard();
                    chrome.runtime.sendMessage({
                        target,
                        method,
                        args,
                    }, resolve);
                });
            };
        },
    }) as any;
}
