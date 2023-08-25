outdir=./out/yomishi-chrome
mkdir -p $outdir
node ./yomishi-ts/bundle_svelte.mjs ./yomishi-ts/extension/popup/index.ts $outdir/popup.js $outdir/popup.css $outdir/popup.html
cp ./yomishi-ts/extension/chrome/manifest.json $outdir/manifest.json