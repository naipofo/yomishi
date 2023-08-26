import { build } from "esbuild";
import { argv } from "process";

const [_node, _script, entrypoint, bundle_js] = argv;

build({
    bundle: true,
    entryPoints: [entrypoint],
    outfile: bundle_js,
    target: ["es6", "chrome77"],
});
