import {
    booleanInterfaceConfig,
    booleanKeys,
    integerInterfaceConfig,
    integerKeys,
    serdeInterfaceConfig,
    serdeKeys,
    serdeType,
    stringInterfaceConfig,
    stringKeys,
} from "@yomishi-config/config";
import { Config } from "@yomishi-proto/config_connect";
import { FetchConfigRequest, PushConfigRequest } from "@yomishi-proto/config_pb";
import { createGenericRpcClient } from "./generic-client";
import { RpcTransport } from "./transport";

// TODO: fetch multiple values from the server at once

type StringKeys = typeof stringKeys[number];
type BoolKeys = typeof booleanKeys[number];
type IntegerKeys = typeof integerKeys[number];
type SerdeKeys = typeof serdeKeys[number];

export type ConfigKeys = StringKeys | BoolKeys | IntegerKeys | SerdeKeys;

export type ConfigType<Type extends ConfigKeys> = Type extends IntegerKeys ? number
    : Type extends StringKeys ? string
    : Type extends BoolKeys ? boolean
    : Type extends SerdeKeys ? serdeType<Type>
    : never;

export function createConfigRpc(transport: RpcTransport) {
    const clinet = createGenericRpcClient(transport, Config);

    const getInterfaceConfig = (key: ConfigKeys) =>
        integerKeys.includes(key as any)
            ? integerInterfaceConfig
            : booleanKeys.includes(key as any)
            ? booleanInterfaceConfig
            : stringKeys.includes(key as any)
            ? stringInterfaceConfig
            : serdeInterfaceConfig;

    return {
        get: async <Key extends ConfigKeys>(key: Key) =>
            JSON.parse(
                (await clinet.fetchConfig(FetchConfigRequest.fromJson({
                    type: getInterfaceConfig(key).type,
                    key,
                }))).config,
            ) as ConfigType<Key>,
        set: <Key extends ConfigKeys>(key: Key, value: ConfigType<Key>) =>
            clinet.pushConfig(PushConfigRequest.fromJson({
                type: getInterfaceConfig(key).type,
                key,
                value: JSON.stringify(value),
            })),
        default: <Key extends ConfigKeys>(key: Key) =>
            (getInterfaceConfig(key).defaultValues as any)[key] as ConfigType<Key>,
    };
}
