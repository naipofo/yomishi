outdir=./out/yomishi-chrome
mkdir -p $outdir
node ./yomishi-ts/bundle_svelte.mjs ./yomishi-ts/extension/action-popup/index.ts $outdir/action-popup.js $outdir/action-popup.css $outdir/action-popup.html
node ./yomishi-ts/bundle_svelte.mjs ./yomishi-ts/extension/popup/index.ts $outdir/popup.js $outdir/popup.css $outdir/popup.html
node ./yomishi-ts/bundle_svelte.mjs ./yomishi-ts/configuration/index.ts $outdir/configuration.js $outdir/configuration.css $outdir/configuration.html
node ./yomishi-ts/bundle_ts.mjs ./yomishi-ts/extension/content-script/content-script.ts $outdir/content-script.js
cp ./yomishi-ts/extension/chrome/manifest.json $outdir/manifest.json
cp ./yomishi-ts/glossary/glolist.css $outdir/glolist.css

node ./yomishi-ts/bundle_ts.mjs ./yomishi-ts/extension/worker/index.ts $outdir/worker.js

cp ./yomishi-ts/extension/offscreen/offscreen.html $outdir/offscreen.html
node ./yomishi-ts/bundle_ts.mjs ./yomishi-ts/extension/offscreen/index.ts $outdir/offscreen.js