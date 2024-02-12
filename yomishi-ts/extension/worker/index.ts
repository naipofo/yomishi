import { addToAnki } from "./anki";

export type BackendRequest = {
    method: keyof BackendHandler;
    args: any[];
};

chrome.runtime.onMessage.addListener((message: BackendRequest, _, respond) => {
    (handler[message.method] as any)(...message.args).then(respond);
    return true;
});

const handler = {
    addToAnki,
} as const;

export type BackendHandler = typeof handler;
