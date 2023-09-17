<script lang="ts">
    export let results: string[];
    console.log(results);
</script>

{#each results as result}
    <article>
        {@html result}
    </article>
{/each}

<style>
    article:not(:last-child) {
        border-bottom: 1px solid black;
        margin-bottom: 10px;
    }
    header {
        font-size: 2rem;
    }
    .rule {
        font-size: 1rem;
    }
    .rule:not(:last-child)::after {
        content: " / ";
    }
    ol:has(li:only-child) {
        list-style: none;
        padding-left: 0;
    }
    li.rendered {
        white-space: pre-line;
    }
</style>
