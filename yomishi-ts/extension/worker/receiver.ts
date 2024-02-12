import { createReceiver } from "../../rpc/chrome-simple";
import { addToAnki } from "./anki";

export const workerReceiver = createReceiver({
    addToAnki,
}, "worker");
