import type { MethodInfo, MethodInfoUnary, ServiceType } from "@bufbuild/protobuf";
import { Message } from "@bufbuild/protobuf";

type RpcClient<T extends ServiceType> = {
    [K in keyof T["methods"]]: T["methods"][K] extends MethodInfoUnary<infer I, infer O> ? (request: I) => Promise<O>
        : never;
};

// TODO: make this cleaner
export function createRpcClient<T extends ServiceType>(service: T, url: string): RpcClient<T> {
    return makeClient(service, url) as RpcClient<T>;
}

type AnyClient = Record<string, AnyClientMethod>;

type AnyClientMethod = (...args: any[]) => any;

function makeClient(
    service: ServiceType,
    url: string,
): AnyClient {
    const client: AnyClient = {};
    for (const [localName, methodInfo] of Object.entries(service.methods)) {
        client[localName] = createMethod(
            methodInfo,
            url,
            service.typeName.split(".").slice(-1)[0],
            methodInfo.name,
        )!;
    }
    return client;
}

function createMethod<I extends Message<I>, O extends Message<O>>(
    info: MethodInfo<I, O>,
    url: string,
    service: string,
    name: string,
): (request: I) => Promise<O> {
    return async (request) => {
        let responce = await fetch(url + "/" + service + "/" + name, {
            "method": "POST",
            "body": request.toBinary(),
        });
        return info.O.fromBinary(new Uint8Array(await responce.arrayBuffer()));
    };
}
