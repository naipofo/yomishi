<script lang="ts">
    import { Anki } from "@yomishi-proto/anki_connect";
    import { ScanMessage } from "../extension/content-script/frames";
    import { createGenericRpcClient } from "../rpc/generic-client";
    import { createLocalServerTransport } from "../rpc/transport";
    import { OpenCardRequest, SaveDefinitionRequest } from "@yomishi-proto/anki_pb";
    import { localAddress } from "../rpc/address-manager";

    export let message: ScanMessage;

    const anki = createGenericRpcClient(createLocalServerTransport(localAddress), Anki);

    let justAdded: number[] = [];

    // TODO: correct order from server
    $: reversed = message.data.results.reverse();
    $: message, (justAdded = []);

    function addToAnki(index: number) {
        justAdded = [...justAdded, index];
        anki.saveDefinition(
            SaveDefinitionRequest.fromJson({
                scanned: message.scanString,
                index: reversed.length - index - 1,
            }),
        );
    }

    function viewInAnki(cid: bigint | undefined) {
        const cId = Number(cid);
        anki.openCard(OpenCardRequest.fromJson({ cId }));
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
