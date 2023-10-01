import { ApiStore } from "../config-store";

export function debounceToStore<T = string>(
    store: ApiStore<T>,
    transform: (v: string) => T = (v) => v as T,
) {
    const timeout = 650;
    let h = 0;
    let callable = (e: { currentTarget: HTMLInputElement }) => {
        clearTimeout(h);
        let val = e.currentTarget.value;
        h = setTimeout(() => store.set(transform(val)), timeout);
    };
    return callable;
}

export function numberTransform(min: number = 0, max: number = Infinity) {
    if (min > max) throw Error();
    return (e: string) => {
        const r = parseInt(e);
        return r < min ? min : r > max ? max : r;
    };
}
