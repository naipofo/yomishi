<script lang="ts">
    import { ScanStringRequest } from "@yomishi-proto/scan_pb";
    import { Scan } from "@yomishi-proto/scan_connect";
    import { createPromiseClient } from "@bufbuild/connect";
    import { createGrpcWebTransport } from "@bufbuild/connect-web";

    const working = "working";
    $: emoji = working == "working" ? "🎉" : "💔";

    const text = "食べさせない";
    const req = ScanStringRequest.fromJson({ text });

    const transport = createGrpcWebTransport({ baseUrl: "http://[::1]:50051" });
    const client = createPromiseClient(Scan, transport);

    const data = client.scanString(req).then((e) => {
        console.log(e);
        return e;
    });
</script>

<div>
    <h1>Yomishi is {working}! {emoji}</h1>
    {#await data then d}
        <code>{d.toJsonString()}</code>
    {/await}
</div>

<style>
    h1 {
        text-decoration: underline;
    }
</style>
