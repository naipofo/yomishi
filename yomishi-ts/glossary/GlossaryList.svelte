<script lang="ts">
    import { Anki } from "@yomishi-proto/anki_connect";
    import { OpenCardRequest } from "@yomishi-proto/anki_pb";
    import {
        localConfigEngine,
        localKeys,
    } from "../configuration/engines/local-storage";
    import { ScanMessage } from "../extension/content-script/frames";
    import { workerClient } from "../extension/worker/client";
    import { createGenericRpcClient } from "../rpc/grcp/generic-client";
    import { createLocalServerTransport } from "../rpc/grcp/transport";

    export let message: ScanMessage;

    const anki = localConfigEngine
        .get(localKeys.localServerAddress)
        .then((e) => createGenericRpcClient(createLocalServerTransport(e), Anki));

    let justAdded: number[] = [];

    // TODO: correct order from server
    $: reversed = message.data.results.reverse();
    $: message, (justAdded = []);

    // TODO: Anki button state for loading
    async function addToAnki(index: number) {
        justAdded = [...justAdded, index];
        await workerClient.addToAnki(
            message.scanString,
            reversed.length - index - 1,
            "seltest",
        );
    }

    async function viewInAnki(cid: bigint | undefined) {
        const cId = Number(cid);
        await (await anki).openCard(OpenCardRequest.fromJson({ cId }));
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
