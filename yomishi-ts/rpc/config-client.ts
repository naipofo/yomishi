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

export function createConfigRpc(transport: RpcTransport) {
    const clinet = createGenericRpcClient(transport, Config);

    const makeInterface = <
        Value extends JsonValue,
        Keys extends readonly string[],
        TypeName extends string,
    >(
        { name, type }: ConfigInterfaceSpec<Value, Keys, TypeName>,
    ):
        & {
            [Prop in `get${TypeName}`]: (key: Keys[number]) => Promise<Value>;
        }
        & {
            [Prop in `set${TypeName}`]: (key: Keys[number], value: Value) => Promise<void>;
        } => ({
            [`get${name}`]: async (key: Keys[number]) =>
                JSON.parse(
                    (await clinet.fetchConfig(FetchConfigRequest.fromJson({
                        type,
                        key,
                    }))).config,
                ) as Value,
            [`set${name}`]: (key: Keys[number], value: Value) =>
                clinet.pushConfig(PushConfigRequest.fromJson({
                    type,
                    key,
                    value: JSON.stringify(value),
                })),
        } as any);

    let a = makeInterface(booleanInterfaceConfig);

    return {
        ...makeInterface(booleanInterfaceConfig),
        ...makeInterface(integerInterfaceConfig),
        ...makeInterface(stringInterfaceConfig),
    };
}
