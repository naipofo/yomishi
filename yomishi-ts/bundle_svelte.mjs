import { build } from "esbuild";
import { sassPlugin } from "esbuild-sass-plugin";
import sveltePlugin from "esbuild-svelte";
import { readFileSync, writeFileSync } from "fs";
import { basename } from "path";
import { argv } from "process";
import sveltePreprocess from "svelte-preprocess";

const [_tsx, _script, entrypoint, bundle_js, _bundle_css, page_html] = argv;

const template = readFileSync("yomishi-ts/page.html", { encoding: "utf8" });
writeFileSync(
    page_html,
    template.replace(/{PAGE}/g, basename(page_html, ".html")),
);

build({
    bundle: true,
    entryPoints: [entrypoint],
    outfile: bundle_js,
    plugins: [
        sassPlugin({
            loadPaths: [".", "node_modules"],
        }),
        sveltePlugin({
            preprocess: [sveltePreprocess({ typescript: true, sourceMap: true })],
        }),
    ],
    sourcemap: "inline",
});
