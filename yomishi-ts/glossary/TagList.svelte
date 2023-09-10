<script lang="ts">
    import { Frequency, Tag } from "@yomishi-proto/scan_pb";

    export let tags: Tag[];
    export let freq: Frequency[] = [];
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

{#each freq as fre}
    <i class="tag">
        &lsqb;
        {fre.name} / {fre.value}
        &rsqb;
    </i>
{/each}

<style>
    i.tag {
        font-size: 1rem;
    }
</style>
