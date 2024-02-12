import type { MethodInfoUnary, ServiceType } from "@bufbuild/protobuf";
import type { RpcTransport } from "./transport";

type RpcClient<T extends ServiceType> = {
    [K in keyof T["methods"]]: T["methods"][K] extends MethodInfoUnary<infer I, infer O> ? (request: I) => Promise<O>
        : never;
};

type AnyClient = Record<string, AnyClientMethod>;
type AnyClientMethod = (...args: any[]) => any;

export function createGenericRpcClient<T extends ServiceType>(
    transport: RpcTransport,
    service: T,
): RpcClient<T> {
    const client: AnyClient = {};
    for (const [localName, methodInfo] of Object.entries(service.methods)) {
        client[localName] = transport(
            service.typeName.split(".").slice(-1)[0],
            methodInfo.name,
            methodInfo,
        );
    }
    return client as RpcClient<T>;
}
