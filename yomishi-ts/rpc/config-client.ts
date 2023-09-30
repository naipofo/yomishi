import { JsonValue } from "@bufbuild/protobuf";
import { boolean_keys, integer_keys, string_keys } from "@yomishi-config/config";
import { Config } from "@yomishi-proto/config_connect";
import { CONFIG_TYPE, FetchConfigRequest, PushConfigRequest } from "@yomishi-proto/config_pb";
import { createGenericRpcClient } from "./generic-client";
import { RpcTransport } from "./transport";

export function createConfigRpc(transport: RpcTransport) {
    const clinet = createGenericRpcClient(transport, Config);

    const makeInterface = <T extends readonly string[], V extends JsonValue, N extends string>(
        type: CONFIG_TYPE,
        name: string,
    ):
        & {
            [Prop in `get${N}`]: (key: T[number]) => Promise<V>;
        }
        & {
            [Prop in `set${N}`]: (key: T[number], value: V) => Promise<void>;
        } => ({
            [`get${name}`]: async (key: T[number]) =>
                JSON.parse(
                    (await clinet.fetchConfig(FetchConfigRequest.fromJson({
                        type,
                        key,
                    }))).config,
                ) as V,
            [`set${name}`]: (key: T[number], value: V) =>
                clinet.pushConfig(PushConfigRequest.fromJson({
                    type,
                    key,
                    value: JSON.stringify(value),
                })),
        } as any);

    // TODO: is the string type param really needed? Maybe ask someone smarter.
    return {
        ...makeInterface<typeof boolean_keys, boolean, "Boolean">(CONFIG_TYPE.BOOLEAN, "Boolean"),
        ...makeInterface<typeof integer_keys, number, "Integer">(CONFIG_TYPE.INTEGER, "Integer"),
        ...makeInterface<typeof string_keys, string, "String">(CONFIG_TYPE.STRING, "String"),
    };
}
