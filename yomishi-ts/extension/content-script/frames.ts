import { ScanStringReply } from "@yomishi-proto/scan_pb";
import { browser } from "../browser-extension";

// TODO: proper API between script and iframe
// including proxing protobuf requests
export type ScanMessage = {
    data: ScanStringReply,
    scanString: string
}

const height = 300;
const width = 600;

function createFrame() {
    const frame = document.createElement("iframe");
    frame.src = browser.runtime.getURL("/popup.html");
    frame.setAttribute(
        "style",
        `width: ${width}px; height: ${height}px; position: absolute; z-index: 999; box-sizing: border-box;`,
    );
    document.body.insertAdjacentElement("beforeend", frame);
    return frame;
}

let frame: HTMLIFrameElement | null = null;

export function updateFrame(data: ScanMessage, scanRect: DOMRect) {
    if (frame == null) {
        frame = createFrame();
    }

    const below = innerHeight - scanRect.y;
    const showAbove = below < height && scanRect.y > below;

    const y = showAbove ? scanRect.y + window.scrollY - height : scanRect.y + scanRect.height + window.scrollY;
    const x = Math.min(scanRect.x, document.body.getBoundingClientRect().width - width);

    frame.style.left = x + "px";
    frame.style.top = y + "px";
    frame.style.display = "block";
    frame.contentWindow?.postMessage(data, "*");
}

document.addEventListener("keydown", e => {
    if (e.key == "Escape" && frame) frame.style.display = "none";
});
