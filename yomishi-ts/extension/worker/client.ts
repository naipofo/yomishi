import { createClient } from "../../rpc/chrome-simple";
import { workerReceiver } from "./receiver";

export const workerClient = createClient(workerReceiver);
