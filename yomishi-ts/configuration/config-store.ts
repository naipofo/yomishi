import { type Readable, writable } from "svelte/store";
import { ConfigKeys, ConfigType, createConfigRpc } from "../rpc/config-client";
import { RpcTransport } from "../rpc/transport";

export type ApiStore<Key extends ConfigKeys> =
    & Readable<
        ConfigValue<ConfigType<Key>>
    >
    & {
        set: (value: ConfigType<Key>) => void;
    };

export type ConfigValue<T> = {
    busy: boolean;
    value: T;
};

export function createConfigStoreProvider(
    transport: RpcTransport,
) {
    const client = createConfigRpc(transport);
    const stores: { [K in ConfigKeys]?: ApiStore<K> } = {};

    const makeStore = <Key extends ConfigKeys>(key: Key): ApiStore<Key> => {
        const { set, update, subscribe } = writable({
            busy: true,
            value: client.default(key),
        });

        client.get(key).then(e =>
            set({
                busy: false,
                value: e,
            })
        );

        return {
            subscribe,
            set: (value: ConfigType<Key>) => {
                set({
                    busy: true,
                    value,
                });
                client.set(key, value).then(() => {
                    set({
                        busy: false,
                        value,
                    });
                });
            },
        };
    };

    return <Key extends ConfigKeys>(key: Key) => {
        return stores[key] || makeStore(key);
    };
}
