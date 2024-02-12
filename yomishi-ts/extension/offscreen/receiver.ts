import { createReceiver } from "../../rpc/chrome-simple";
import { clipboardText } from "./clipboard";

export const offscreenReceiver = createReceiver({
    clipboardText,
}, "offscreen");
