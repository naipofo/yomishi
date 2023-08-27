<script lang="ts">
    import { RubySegment } from "@yomishi-proto/scan_pb";

    export let string: RubySegment[];
</script>

{#each string as part}
    {#if part.ruby}
        <ruby>
            {part.text}
            <rt>
                {part.ruby}
            </rt>
        </ruby>
    {:else}
        {part.text}
    {/if}
{/each}

<style>
</style>
