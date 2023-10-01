import { JsonValue } from "@bufbuild/protobuf";
import {
    booleanInterfaceConfig,
    ConfigInterfaceSpec,
    integerInterfaceConfig,
    serdeInterfaceConfig,
    serdeKeys,
    serdeType,
    stringInterfaceConfig,
} from "@yomishi-config/config";
import { Config } from "@yomishi-proto/config_connect";
import { CONFIG_TYPE, FetchConfigRequest, PushConfigRequest } from "@yomishi-proto/config_pb";
import { createGenericRpcClient } from "./generic-client";
import { RpcTransport } from "./transport";

// TODO: fetch multiple values from the server at once

export type AsyncGetSet<Key, Value> = {
    get: (key: Key) => Promise<Value>;
    set: (key: Key, value: Value) => Promise<void>;
    default: (key: Key) => Value;
};

type SerdeKey = typeof serdeKeys[number];
export type SerdeAsyncGetSet = {
    get: <Key extends SerdeKey>(key: Key) => Promise<serdeType<Key>>;
    set: <Key extends SerdeKey>(key: Key, value: serdeType<Key>) => Promise<void>;
    default: <Key extends SerdeKey>(key: Key) => serdeType<Key>;
};

export function createConfigRpc(transport: RpcTransport) {
    const clinet = createGenericRpcClient(transport, Config);

    const get = async (key: string, type: CONFIG_TYPE) =>
        JSON.parse(
            (await clinet.fetchConfig(FetchConfigRequest.fromJson({
                type,
                key,
            }))).config,
        );
    const set = (key: string, type: CONFIG_TYPE, value: JsonValue) =>
        clinet.pushConfig(PushConfigRequest.fromJson({
            type,
            key,
            value: JSON.stringify(value),
        }));

    const makePlainInterface = <
        Value extends JsonValue,
        Keys extends readonly string[],
        TypeName extends string,
    >(
        { name, type, defaultValues }: ConfigInterfaceSpec<Value, Keys, TypeName>,
    ): {
        [Prop in TypeName]: AsyncGetSet<Keys[number], Value>;
    } => ({
        [name]: {
            get: async (key: Keys[number]) => get(key, type),
            set: (key: Keys[number], value: Value) => set(key, type, value),
            default: (key: Keys[number]) => defaultValues[key],
        },
    } as any);

    return {
        ...makePlainInterface(booleanInterfaceConfig),
        ...makePlainInterface(integerInterfaceConfig),
        ...makePlainInterface(stringInterfaceConfig),
        ...(makePlainInterface(serdeInterfaceConfig) as { any: SerdeAsyncGetSet }),
    };
}
