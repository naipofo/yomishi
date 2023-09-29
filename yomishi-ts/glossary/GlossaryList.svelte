<script lang="ts">
    import { ScanMessage } from "../extension/content-script/frames";

    export let message: ScanMessage;

    let justAdded: number[] = [];

    // TODO: correct order from server
    $: reversed = message.data.results.reverse();
    $: message, (justAdded = []);

    function addToAnki(index: number) {
        // TODO: Anki integration
        justAdded = [...justAdded, index];
    }

    function viewInAnki(cid: bigint | undefined) {
        // TODO: Anki integration
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
