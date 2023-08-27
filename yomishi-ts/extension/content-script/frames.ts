import { ScanStringReply } from "@yomishi-proto/scan_pb";
import { browser } from "../browser-extension";

function createFrame() {
    const frame = document.createElement("iframe");
    frame.src = browser.runtime.getURL("/popup.html");
    frame.setAttribute("style", `width: 400px; height: 400px; position: absolute; z-index: 999;`);
    document.body.insertAdjacentElement("beforeend", frame);
    return frame;
}

let frame: HTMLIFrameElement | null = null;

export function updateFrame(data: ScanStringReply, x: number, y: number) {
    if (frame == null) {
        frame = createFrame();
    }
    frame.style.left = x + "px";
    frame.style.top = y + "px";
    frame.contentWindow?.postMessage(data, "*");
}
