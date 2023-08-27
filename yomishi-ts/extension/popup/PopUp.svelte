<script lang="ts">
    import { ScanStringReply } from "@yomishi-proto/scan_pb";
    import GlossaryList from "../../glossary/GlossaryList.svelte";

    let data: ScanStringReply | null = null;
    function messageHandler(e: MessageEvent<ScanStringReply>) {
        data = e.data;
    }
</script>

<svelte:window on:message={messageHandler} />
{#if data}
    <GlossaryList glossary={data.results} />
{:else}
    <h2>Loading...</h2>
{/if}

<style>
    :global(body) {
        background: white;
    }
</style>
