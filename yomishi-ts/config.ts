import { boolean_keys, integer_keys, string_keys } from "@yomishi-config/config";
import { Config } from "@yomishi-proto/config_connect";
import { CONFIG_TYPE, FetchConfigRequest } from "@yomishi-proto/config_pb";
import { createRpcClient } from "./rpc_transport";
console.log(boolean_keys, integer_keys, string_keys);

export function createConfigRpc(url: string) {
    const clinet = createRpcClient(Config, url);
    const getVal = <T extends readonly string[], R>(type: CONFIG_TYPE) => async (key: T[number]) =>
        JSON.parse(
            (await clinet.fetchConfig(FetchConfigRequest.fromJson({
                type,
                key,
            }))).config,
        ) as R;
    return {
        getBoolean: getVal<typeof boolean_keys, boolean>(CONFIG_TYPE.BOOLEAN),
        getInteger: getVal<typeof integer_keys, number>(CONFIG_TYPE.INTEGER),
        getString: getVal<typeof string_keys, string>(CONFIG_TYPE.STRING),
    };
}
