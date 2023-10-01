import { type Readable, writable } from "svelte/store";
import { ConfigKeys, ConfigType, createConfigRpc } from "../rpc/config-client";
import { RpcTransport } from "../rpc/transport";

export type ApiStore<T> =
    & Readable<ConfigValue<T>>
    & {
        set: (value: T) => void;
        // TODO: reset to default + isDefault
    };

export type ConfigValue<T> = {
    busy: boolean;
    value: T;
};

export function createConfigStoreProvider(
    transport: RpcTransport,
) {
    const client = createConfigRpc(transport);
    const stores: { [K in ConfigKeys]?: ApiStore<ConfigType<K>> } = {};

    const makeStore = <Key extends ConfigKeys>(key: Key): ApiStore<ConfigType<Key>> => {
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

        const store = {
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
        (stores as any)[key] = store;
        return store;
    };

    return <Key extends ConfigKeys>(key: Key) => {
        if (!stores[key]) {
            console.log("creating store", key);
        }
        return stores[key] || makeStore(key);
    };
}
