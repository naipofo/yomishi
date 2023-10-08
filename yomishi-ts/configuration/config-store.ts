import { type Readable, writable } from "svelte/store";
import { ConfigEngine, NamedKey } from "./engines";

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

export const createStoreGenerator = <Keys>(engine: ConfigEngine<Keys>) => {
    const stores: Record<string, ApiStore<any>> = {};

    const makeStore = <Value>(key: NamedKey<Value, Keys>): ApiStore<Value> => {
        const { set, subscribe } = writable({
            busy: true,
            value: engine.default(key),
        });

        engine.get(key).then(e =>
            set({
                busy: false,
                value: e,
            })
        );

        const store: ApiStore<Value> = {
            subscribe,
            set: (value) => {
                set({
                    busy: true,
                    value,
                });
                engine.set(key, value).then(() => {
                    set({
                        busy: false,
                        value,
                    });
                });
            },
        };
        stores[key.name] = store;
        return store;
    };

    return <V>(key: NamedKey<V, Keys>): ApiStore<V> => {
        return stores[key.name] || makeStore(key);
    };
};
