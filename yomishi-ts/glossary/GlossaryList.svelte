<script lang="ts">
    import { ScanResult } from "@yomishi-proto/scan_pb";
    import RubyRender from "./RubyRender.svelte";
    import TagList from "./TagList.svelte";

    export let results: ScanResult[];
    console.log(results);
</script>

{#each results as result}
    <article>
        <header>
            <RubyRender string={result.ruby} />
            <span>
                {#each result.inflectionRules as rule}
                    <span class="rule">{rule}</span>
                {/each}
            </span>
            <TagList tags={result.tags} freq={result.frequency} />
        </header>
        <ol>
            <li>
                {#each result.glossary as glossary}
                    <TagList tags={glossary.tags} />
                    <ul>
                        {#each glossary.definition as def}
                            <li class="rendered">{@html def}</li>
                        {/each}
                    </ul>
                {/each}
            </li>
        </ol>
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
