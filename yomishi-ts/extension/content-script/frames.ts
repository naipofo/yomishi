import { rpcKeys } from "@yomishi-config/config";
import { ScanStringReply } from "@yomishi-proto/scan_pb";
import { cachedEngine } from "../../configuration/engines/cached";
import { createConfigRpcEngine } from "../../configuration/engines/config-rpc";
import { localConfigEngine, localKeys } from "../../configuration/engines/local-storage";
import { createLocalServerTransport } from "../../rpc/transport";
import { browser } from "../browser-extension";

// TODO: proper API between script and iframe
// including proxing protobuf requests
export type ScanMessage = {
    data: ScanStringReply;
    scanString: string;
};

const config = localConfigEngine.get(localKeys.localServerAddress)
    .then(e => cachedEngine(createConfigRpcEngine(createLocalServerTransport(e))));

function createFrame() {
    const frame = document.createElement("iframe");
    frame.src = browser.runtime.getURL("/popup.html");
    frame.setAttribute(
        "style",
        `width: 600px; height: 400px; position: absolute; z-index: 999; box-sizing: border-box;`,
    );
    document.body.insertAdjacentElement("beforeend", frame);
    return frame;
}

let frame: HTMLIFrameElement | null = null;

export async function updateFrame(data: ScanMessage, scanRect: DOMRect) {
    const bodyRect = document.body.getBoundingClientRect();

    const width = Math.min(
        await (await config).get(rpcKeys.PopupWidth),
        // TODO: only reload when configuration changed
        bodyRect.width,
    );
    // TODO: height should be constrained to space, not body height
    const height = Math.min(
        await (await config).get(rpcKeys.PopupHeight),
        bodyRect.height,
    );

    if (frame == null) {
        frame = createFrame();
    }

    const below = innerHeight - scanRect.y;
    const showAbove = below < height && scanRect.y > below;

    const y = showAbove ? scanRect.y + window.scrollY - height : scanRect.y + scanRect.height + window.scrollY;
    const x = Math.min(scanRect.x, bodyRect.width - width);

    frame.contentWindow?.postMessage(data, "*");

    frame.style.left = x + "px";
    frame.style.top = y + "px";

    frame.style.width = width + "px";
    frame.style.height = height + "px";

    frame.style.display = "block";
}

document.addEventListener("keydown", e => {
    if (e.key == "Escape" && frame) frame.style.display = "none";
});
