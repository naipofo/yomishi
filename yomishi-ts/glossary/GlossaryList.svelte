<script lang="ts">
    import { createPromiseClient } from "@bufbuild/connect";
    import { createGrpcWebTransport } from "@bufbuild/connect-web";
    import { Anki } from "@yomishi-proto/anki_connect";
    import { ScanMessage } from "../extension/content-script/frames";

    export let message: ScanMessage;

    // TODO: correct order from server
    $: reversed = message.data.results.reverse();

    function addToAnki(index: number) {
        const transport = createGrpcWebTransport({ baseUrl: "http://[::1]:50051" });
        const client = createPromiseClient(Anki, transport);
        client.saveDefinition({
            scanned: message.scanString,
            index: reversed.length - index - 1,
            // And then remove this
        });
    }
</script>

{#each reversed as result, i}
    <article>
        <div class="buttons">
            <button on:click={() => addToAnki(i)}>anki</button>
        </div>
        {@html result}
    </article>
{/each}

<svelte:head>
    <link rel="stylesheet" type="text/css" href="glolist.css" media="all" />
</svelte:head>

<style lang="scss">
    @use "yomishi-styles/tag-colors";

    article:not(:last-child) {
        border-bottom: 1px solid black;
        margin-bottom: 10px;
    }
    .buttons {
        float: right;
    }
</style>
