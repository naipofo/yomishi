import { JsonValue } from "@bufbuild/protobuf";
import {
    booleanInterfaceConfig,
    ConfigInterfaceSpec,
    integerInterfaceConfig,
    stringInterfaceConfig,
} from "@yomishi-config/config";
import { type Writable, writable } from "svelte/store";
import { createConfigRpc } from "../rpc/config-client";
import { RpcTransport } from "../rpc/transport";

type ApiStore<T> = Omit<Writable<T | null>, "update">;

export function createConfigStoreProvider(
    transport: RpcTransport,
) {
    // TODO: remove some any casts and clean up code

    const client = createConfigRpc(transport);
    const stores: Record<string, ApiStore<any>> = {};

    const createStore = <K>(
        key: string,
        type_name: keyof typeof client,
    ): ApiStore<K> => {
        const store: Writable<any | null> = writable(null);

        (client[type_name].get as any)(key).then((e: K) => {
            store.set(e);
        });

        return {
            subscribe: store.subscribe as any,
            set: (value) => {
                if (value === null) return;
                store.set(null);
                (client[type_name].set as any)(key).then(() => {
                    store.set(value);
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
        [name]: (key: Keys[number]) => {
            fetchStore<Value>(key, name as any);
        },
    } as any);

    return {
        ...makeInterface(booleanInterfaceConfig),
        ...makeInterface(integerInterfaceConfig),
        ...makeInterface(stringInterfaceConfig),
    };
}
