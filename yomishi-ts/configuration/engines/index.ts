import { ConfigKey } from "@yomishi-config/config";

export interface ConfigEngine<Keys> {
    get<G>(key: NamedKey<G, Keys>): Promise<G>;
    set<G>(key: NamedKey<G, Keys>, value: G): Promise<void>;
    default<G>(key: NamedKey<G, Keys>): G;
}
export type NamedKey<Value, Keys> = ConfigKey<Value> & { name: Keys };
