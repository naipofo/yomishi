import { Anki } from "@yomishi-proto/anki_connect";
import { SaveDefinitionRequest } from "@yomishi-proto/anki_pb";
import { localConfigEngine, localKeys } from "../../configuration/engines/local-storage";
import { createGenericRpcClient } from "../../rpc/generic-client";
import { createLocalServerTransport } from "../../rpc/transport";

export async function addToAnki(scanned: string, index: number, selection: string) {
    const anki = createGenericRpcClient(
        createLocalServerTransport(await localConfigEngine.get(localKeys.localServerAddress)),
        Anki,
    );

    await chrome.offscreen.createDocument({
        url: "offscreen.html",
        reasons: [chrome.offscreen.Reason.CLIPBOARD],
        justification: "accessing clipboard",
    });

    const clipboard: string = await new Promise((r) =>
        chrome.runtime.sendMessage({
            target: "offscreen",
            data: "",
        }, r)
    );

    await anki.saveDefinition(SaveDefinitionRequest.fromJson({
        scanned,
        index,
        state: {
            clipboard,
            selection,
        },
    }));
}
