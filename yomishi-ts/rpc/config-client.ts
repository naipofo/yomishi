import { JsonValue } from "@bufbuild/protobuf";
import {
    booleanInterfaceConfig,
    ConfigInterfaceSpec,
    integerInterfaceConfig,
    stringInterfaceConfig,
} from "@yomishi-config/config";
import { Config } from "@yomishi-proto/config_connect";
import { FetchConfigRequest, PushConfigRequest } from "@yomishi-proto/config_pb";
import { createGenericRpcClient } from "./generic-client";
import { RpcTransport } from "./transport";

// TODO: incorporate default values from spec
// should be shown as disabled before the real ones get loaded from server

export type AsyncGetSet<Key, Value> = {
    get: (key: Key) => Promise<Value>;
    set: (key: Key, value: Value) => Promise<void>;
};

export function createConfigRpc(transport: RpcTransport) {
    const clinet = createGenericRpcClient(transport, Config);

    const makeInterface = <
        Value extends JsonValue,
        Keys extends readonly string[],
        TypeName extends string,
    >(
        { name, type }: ConfigInterfaceSpec<Value, Keys, TypeName>,
    ): {
        [Prop in TypeName]: AsyncGetSet<Keys[number], Value>;
    } => ({
        [name]: {
            get: async (key: Keys[number]) =>
                JSON.parse(
                    (await clinet.fetchConfig(FetchConfigRequest.fromJson({
                        type,
                        key,
                    }))).config,
                ) as Value,
            set: (key: Keys[number], value: Value) =>
                clinet.pushConfig(PushConfigRequest.fromJson({
                    type,
                    key,
                    value: JSON.stringify(value),
                })),
        },
    } as any);

    return {
        ...makeInterface(booleanInterfaceConfig),
        ...makeInterface(integerInterfaceConfig),
        ...makeInterface(stringInterfaceConfig),
    };
}
