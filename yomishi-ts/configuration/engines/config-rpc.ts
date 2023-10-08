import { ConfigKey, ProtoTyped, rpcKeys } from "@yomishi-config/config";
import { Config } from "@yomishi-proto/config_connect";
import { FetchConfigRequest, PushConfigRequest } from "@yomishi-proto/config_pb";
import { createGenericRpcClient } from "../../rpc/generic-client";
import { RpcTransport } from "../../rpc/transport";
import { ConfigEngine } from ".";

// TODO: fetch multiple values from the server at once

export function createConfigRpcEngine(transport: RpcTransport) {
    const clinet = createGenericRpcClient(transport, Config);

    return {
        get: async <T>({ name, type }: ConfigKey<T> & ProtoTyped) =>
            JSON.parse(
                (await clinet.fetchConfig(FetchConfigRequest.fromJson({
                    type,
                    key: name,
                }))).config,
            ) as T,
        set: async <T>({ name, type }: ConfigKey<T> & ProtoTyped, value: T) => {
            await clinet.pushConfig(PushConfigRequest.fromJson({
                type,
                key: name,
                value: JSON.stringify(value),
            }));
        },
        default: <T>(key: ConfigKey<T>) => key.default,
    } as ConfigEngine<typeof rpcKeys[keyof typeof rpcKeys]["name"]>;
}
