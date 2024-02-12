import { createClient } from "../../rpc/chrome-simple";
import { offscreenReceiver } from "./receiver";

async function ensureCreated() {
    if (!(await hasOffscreenDocument())) {
        await chrome.offscreen.createDocument({
            url: "offscreen.html",
            reasons: [chrome.offscreen.Reason.CLIPBOARD],
            justification: "accessing clipboard",
        });
    }
}

async function hasOffscreenDocument() {
    if ("getContexts" in chrome.runtime) {
        const contexts = await (chrome.runtime as any).getContexts({
            contextTypes: ["OFFSCREEN_DOCUMENT"],
            documentUrls: ["offscreen.html"],
        });
        return Boolean(contexts.length);
    } else {
        const matchedClients = await (self as any).clients.matchAll();
        return await matchedClients.some((client: any) => {
            client.url.includes(chrome.runtime.id);
        });
    }
}

export const offscreenClient = createClient(offscreenReceiver, ensureCreated);
