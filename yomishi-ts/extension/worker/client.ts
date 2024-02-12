import { BackendHandler, BackendRequest } from ".";

export const backendClient = new Proxy({}, {
    get(_, p: keyof BackendHandler) {
        return (...args: any[]) => {
            return new Promise((resolve) => {
                chrome.runtime.sendMessage({
                    method: p,
                    args,
                } as BackendRequest, resolve);
            });
        };
    },
}) as BackendHandler;
