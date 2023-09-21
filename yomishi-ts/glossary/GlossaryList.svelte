<script lang="ts">
    import { createPromiseClient } from "@bufbuild/connect";
    import { createGrpcWebTransport } from "@bufbuild/connect-web";
    import { Anki } from "@yomishi-proto/anki_connect";
    import { ScanMessage } from "../extension/content-script/frames";

    export let message: ScanMessage;

    let justAdded: number[] = [];

    // TODO: correct order from server
    $: reversed = message.data.results.reverse();
    $: message, (justAdded = []);

    const transport = createGrpcWebTransport({ baseUrl: "http://[::1]:50051" });
    const client = createPromiseClient(Anki, transport);

    function addToAnki(index: number) {
        justAdded = [...justAdded, index];

        client.saveDefinition({
            scanned: message.scanString,
            index: reversed.length - index - 1,
            // And then remove this
        });
    }

    function viewInAnki(cid: bigint | undefined) {
        client.openCard({
            cId: cid,
        });
    }
</script>

{#each reversed as result, i}
    <article>
        <div class="buttons">
            {#if result.cardId != undefined}
                <button on:click={() => viewInAnki(result.cardId)}>card</button>
            {/if}
            <button
                on:click={() => addToAnki(i)}
                disabled={!result.ankiCanAdd || justAdded.indexOf(i) !== -1}
            >
                anki
            </button>
        </div>
        {@html result.content}
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
