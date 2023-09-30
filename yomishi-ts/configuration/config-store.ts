import { JsonValue } from "@bufbuild/protobuf";
import {
    booleanInterfaceConfig,
    ConfigInterfaceSpec,
    integerInterfaceConfig,
    stringInterfaceConfig,
} from "@yomishi-config/config";
import { type Readable, type Writable, writable } from "svelte/store";
import { AsyncGetSet, createConfigRpc } from "../rpc/config-client";
import { RpcTransport } from "../rpc/transport";

export type ApiStore<T> = Readable<ConfigValue<T>> & {
    set: (value: T) => void;
};

export type ConfigValue<T> = {
    busy: boolean;
    value: T;
};

export function createConfigStoreProvider(
    transport: RpcTransport,
) {
    // TODO: remove some any casts and clean up code

    const client = createConfigRpc(transport);
    const stores: Record<string, ApiStore<any>> = {};

    const createStore = <V>(
        key: string,
        type_name: keyof typeof client,
    ): ApiStore<V> => {
        const api: AsyncGetSet<typeof key, V> = client[type_name] as any;

        const store: Writable<ConfigValue<V>> = writable({
            busy: true,
            value: api.default(key),
        });

        api.get(key).then(e => {
            store.set({
                busy: false,
                value: e,
            });
        });

        return {
            subscribe: store.subscribe as any,
            set: (value) => {
                store.set({
                    busy: true,
                    value,
                });
                api.set(key, value).then(() => {
                    store.set({
                        busy: false,
                        value,
                    });
                });
            },
        };
    };

    const fetchStore = <Value>(key: string, type_name: keyof typeof client): ApiStore<Value> => {
        let s = stores[key];
        if (s === undefined) {
            stores[key] = createStore(key, type_name) as any;
            return fetchStore(key, type_name);
        }
        return s;
    };

    const makeInterface = <
        Value extends JsonValue,
        Keys extends readonly string[],
        TypeName extends string,
    >({ name }: ConfigInterfaceSpec<Value, Keys, TypeName>): {
        [Prop in TypeName]: (key: Keys[number]) => ApiStore<Value>;
    } => ({
        [name]: (key: Keys[number]) => fetchStore<Value>(key, name as any),
    } as any);

    return {
        ...makeInterface(booleanInterfaceConfig),
        ...makeInterface(integerInterfaceConfig),
        ...makeInterface(stringInterfaceConfig),
    };
}
