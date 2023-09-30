import { boolean_keys } from "@yomishi-config/config";
import { type Writable, writable } from "svelte/store";
import { createConfigRpc } from "../rpc/config-client";
import { RpcTransport } from "../rpc/transport";

type ApiStore<T> = Omit<Writable<T | null>, "update">;

export function createConfigStoreProvider(
    transport: RpcTransport,
) {
    // TODO: Share types with `createConfigRpc`
    // TODO: remove some any casts

    const client = createConfigRpc(transport);
    const stores: Partial<{ [K in string]: ApiStore<K> }> = {};

    const createStore = <K extends string>(
        key: K,
        type_name: string,
    ): ApiStore<K> => {
        const store: Writable<any | null> = writable(null);

        (client as any)[`get${type_name}`](key).then((e: any) => {
            store.set(e);
        });

        return {
            subscribe: store.subscribe as any,
            set: (value) => {
                if (value === null) return;
                store.set(null);
                (client as any)[`set${type_name}`](key, value).then(() => {
                    store.set(value as any);
                });
            },
        };
    };

    const fetchStore = (key: string, type_name: string): any => {
        let s = stores[key];
        if (s === undefined) {
            stores[key] = createStore(key, type_name) as any;
            return fetchStore(key, type_name);
        }
        return s;
    };

    return {
        createBoolStore(key: typeof boolean_keys[number]): ApiStore<boolean> {
            return fetchStore(key, "Boolean");
        },
    };
}
