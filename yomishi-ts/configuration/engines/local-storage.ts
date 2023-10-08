import { browser } from "../../extension/browser-extension";
import { ConfigEngine } from ".";

export const localKeys = {
    localServerAddress: {
        name: "LocalServerAddress" as const,
        default: "http://127.0.0.1:50051",
    },
};

export const localConfigEngine: ConfigEngine<typeof localKeys[keyof typeof localKeys]["name"]> = {
    async get(key) {
        return (await browser.storage.local.get([key.name]) as any)[key.name]
            || key.default;
    },
    async set(key, value) {
        await browser.storage.local.set({ [key.name]: value });
    },
    default(key) {
        return key.default;
    },
};
