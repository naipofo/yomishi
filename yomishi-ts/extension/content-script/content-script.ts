import { createPromiseClient } from "@bufbuild/connect";
import { createGrpcWebTransport } from "@bufbuild/connect-web";
import { Scan } from "@yomishi-proto/scan_connect";
import { ScanStringReply, ScanStringRequest } from "@yomishi-proto/scan_pb";
import { browser } from "../browser-extension";

console.log("yomishi init!");

let lastScan = "";
let isScanning = false;

document.body.addEventListener("mousemove", async (e) => {
    if (!isScanning) {
        isScanning = true;
        await scanFromEvent(e);
        isScanning = false;
    }
});

async function scanFromEvent(e: MouseEvent) {
    const hoveredString = getStringFromCaret(e.clientX, e.clientY).trim().substring(0, 16);
    if (hoveredString != lastScan) {
        lastScan = hoveredString;

        const req = ScanStringRequest.fromJson({ text: lastScan });

        const transport = createGrpcWebTransport({ baseUrl: "http://[::1]:50051" });
        const client = createPromiseClient(Scan, transport);

        const data = await client.scanString(req);
        updateFrame(data, e.clientX, e.clientY);
    }
}

function getStringFromCaret(x: number, y: number): string {
    const anyDocument = document as any;
    let range: any;
    let offset: number;
    let textNode;

    if (typeof anyDocument.caretRangeFromPoint === "function") {
        // Chromium
        range = anyDocument.caretRangeFromPoint(x, y);
        offset = range.startOffset;
        textNode = range.startContainer;
    } else if (typeof anyDocument.caretPositionFromPoint === "function") {
        // Firefox
        range = anyDocument.caretPositionFromPoint(x, y);
        offset = range.offset;
        textNode = range.offsetNode;
    } else {
        throw new Error("carret support not found!");
    }
    let data = textNode.textContent as string;
    // TODO: break node boudaries
    return data.substring(offset, data.length);
}

const frame = document.createElement("iframe");
frame.src = browser.runtime.getURL("/popup.html");
frame.setAttribute("style", `width: 400px; height: 400px; position: absolute; top: 1px; left: 1px; z-index: 999;`);
document.body.insertAdjacentElement("beforeend", frame);

function updateFrame(data: ScanStringReply, x: number, y: number) {
    frame.style.left = x + "px";
    frame.style.top = y + "px";
    frame.contentWindow?.postMessage(data, "*");
}
