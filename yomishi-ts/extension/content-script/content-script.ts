import { Scan } from "@yomishi-proto/scan_connect";
import { ScanStringRequest } from "@yomishi-proto/scan_pb";
import { createGenericRpcClient } from "../../rpc/generic-client";
import { createLocalServerTransport } from "../../rpc/transport";
import { updateFrame } from "./frames";

console.log("yomishi init!");

let lastScan = "";
let isScanning = false;

let rpcTransport = createLocalServerTransport("http://127.0.0.1:50051");

document.body.addEventListener("mousemove", async (e) => {
    if (!isScanning) {
        isScanning = true;
        await scanFromEvent(e);
        isScanning = false;
    }
});

async function scanFromEvent(e: MouseEvent) {
    const [str, rect] = getStringFromCaret(e.clientX, e.clientY);

    const hoveredString = str.trim().substring(0, 16);
    if (hoveredString != lastScan && hoveredString && rect) {
        lastScan = hoveredString;

        const req = ScanStringRequest.fromJson({ text: lastScan });

        const client = createGenericRpcClient(rpcTransport, Scan);

        const data = await client.scanString(req);
        if (data.results.length > 0) {
            // TODO: detect vertical text
            updateFrame({ data, scanString: lastScan }, rect);
        }
    }
}

function getStringFromCaret(x: number, y: number): [string, DOMRect] {
    const anyDocument = document as any;
    let range: any;
    let offset: number;
    let textNode: Text;
    let rect: DOMRect;

    if (typeof anyDocument.caretRangeFromPoint === "function") {
        // Chromium
        range = anyDocument.caretRangeFromPoint(x, y);
        offset = range.startOffset;
        textNode = range.startContainer;
        rect = range.getClientRects()[0];
    } else if (typeof anyDocument.caretPositionFromPoint === "function") {
        // Firefox
        range = anyDocument.caretPositionFromPoint(x, y);
        offset = range.offset;
        textNode = range.offsetNode;
        rect = range.getClientRect();
    } else {
        throw new Error("carret support not found!");
    }
    let data = textNode.textContent as string;

    // TODO: break node boudaries
    return [data.substring(offset, data.length), rect];
}
