import { ConfigEngine } from ".";

export function cachedEngine<K>(engine: ConfigEngine<K>): ConfigEngine<K> {
    const { set, get, default: def } = engine;
    const cache: Record<string, any> = {};
    return {
        get: new Proxy(get, {
            async apply(target, thisArg, argArray) {
                const key = argArray[0].name;
                if (!cache[key]) {
                    const value = await Reflect.apply(target, thisArg, argArray);
                    cache[key] = value;
                }
                return cache[key];
            },
        }),
        set: new Proxy(set, {
            apply(target, thisArg, argArray) {
                cache[argArray[0].name] = argArray[1];
                return Reflect.apply(target, thisArg, argArray);
            },
        }),
        default: def,
    };
}
