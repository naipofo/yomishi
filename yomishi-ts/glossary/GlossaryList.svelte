<script lang="ts">
    export let results: string[];
    console.log(results);
</script>

{#each results as result}
    <article>
        <div class="buttons">
            <button>anki</button>
        </div>
        {@html result}
    </article>
{/each}

<svelte:head>
    <link rel="stylesheet" type="text/css" href="glolist.css" media="all" />
</svelte:head>

<style>
    article:not(:last-child) {
        border-bottom: 1px solid black;
        margin-bottom: 10px;
    }
    .buttons {
        float: right;
    }
</style>
