import { getContext, setContext } from "svelte";

export abstract class ScaffoldContext {
    static key: string = "ScaffoldContext";

    setCurrentContext() {
        setContext(ScaffoldContext.key, this);
    }
    static getCurrentContext(): ScaffoldContext | null {
        return getContext(ScaffoldContext.key);
    }

    abstract registerSection(name: string): void;
}
