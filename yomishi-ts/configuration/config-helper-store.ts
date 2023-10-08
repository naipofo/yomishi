import { Config } from "@yomishi-proto/config_connect";
import { AnkiConfigDataReply, AnkiConfigDataRequest } from "@yomishi-proto/config_pb";
import { writable } from "svelte/store";
import { createGenericRpcClient } from "../rpc/generic-client";
import { RpcTransport } from "../rpc/transport";

export function createConfigHelperStore(transport: RpcTransport) {
    const { set, subscribe } = writable(new AnkiConfigDataReply());
    const client = createGenericRpcClient(transport, Config);

    return {
        subscribe,
        refresh: () =>
            client.ankiConfigData(AnkiConfigDataRequest.fromJson({}))
                .then(set),
    };
}
