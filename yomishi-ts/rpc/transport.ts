import { Message, MethodInfo } from "@bufbuild/protobuf";

export type RpcTransport = <I extends Message<I>, O extends Message<O>>(
    service: string,
    method: string,
    info: MethodInfo<I, O>,
) => (request: I) => Promise<O>;

export function createLocalServerTransport(url: string): RpcTransport {
    return <I extends Message<I>, O extends Message<O>>(
        service: string,
        method: string,
        info: MethodInfo<I, O>,
    ) => {
        return async (request) => {
            let responce = await fetch(url + "/" + service + "/" + method, {
                "method": "POST",
                "body": request.toBinary(),
            });
            return info.O.fromBinary(new Uint8Array(await responce.arrayBuffer()));
        };
    };
}
