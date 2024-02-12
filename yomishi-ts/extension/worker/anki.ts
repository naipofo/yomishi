import { Anki } from "@yomishi-proto/anki_connect";
import { SaveDefinitionRequest } from "@yomishi-proto/anki_pb";
import { localConfigEngine, localKeys } from "../../configuration/engines/local-storage";
import { createGenericRpcClient } from "../../rpc/grcp/generic-client";
import { createLocalServerTransport } from "../../rpc/grcp/transport";
import { offscreenClient } from "../offscreen/client";

export async function addToAnki(scanned: string, index: number, selection: string) {
    const anki = createGenericRpcClient(
        createLocalServerTransport(await localConfigEngine.get(localKeys.localServerAddress)),
        Anki,
    );

    const clipboard = await offscreenClient.clipboardText();

    await anki.saveDefinition(SaveDefinitionRequest.fromJson({
        scanned,
        index,
        state: {
            clipboard,
            selection,
        },
    }));
}
