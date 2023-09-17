<script lang="ts">
    import GlossaryList from "../../glossary/GlossaryList.svelte";
    import { ScanMessage } from "../content-script/frames";

    let message: ScanMessage | null = null;

    function messageHandler(e: MessageEvent<ScanMessage>) {
        message = e.data;
    }
</script>

<svelte:window on:message={messageHandler} />
{#if message}
    <GlossaryList {message} />
{:else}
    <h2>Loading...</h2>
{/if}

<style>
    :global(body) {
        background: white;
    }
</style>
