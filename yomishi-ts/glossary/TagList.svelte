<script lang="ts">
    import { Tag } from "@yomishi-proto/scan_pb";

    export let tags: Tag[];
</script>

{#each tags as tag}
    <!-- svelte-ignore a11y-click-events-have-key-events -->
    <!-- svelte-ignore a11y-no-static-element-interactions -->
    <i
        class="tag"
        on:click={(_) => {
            alert(tag.description);
            // TODO: custom popup instead of alerts
        }}
    >
        &lsqb;
        {tag.name}
        &rsqb;
    </i>
{/each}

<style>
    i.tag {
        font-size: 1rem;
    }
</style>
