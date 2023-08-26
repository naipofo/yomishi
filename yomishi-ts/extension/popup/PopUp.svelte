<script lang="ts">
    import { ScanStringReply } from "@yomishi-proto/scan_pb";

    let data: ScanStringReply | null = null;
    function messageHandler(e: MessageEvent<ScanStringReply>) {
        data = e.data;
    }
</script>

<svelte:window on:message={messageHandler} />
{#if data}
    {#each data.results as result}
        <div>
            <h4>{result.expression}</h4>
            {#each result.glossary as def}
                <p>{def}</p>
            {/each}
        </div>
    {/each}
{/if}

<style>
    div:not(:last-child) {
        border-bottom: 1px solid black;
        margin-bottom: 10px;
    }
    :global(body) {
        background: white;
    }
</style>
