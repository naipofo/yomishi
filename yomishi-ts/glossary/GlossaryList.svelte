<script lang="ts">
    import { ScanResult } from "@yomishi-proto/scan_pb";
    import RubyRender from "./RubyRender.svelte";

    export let glossary: ScanResult[];
</script>

{#each glossary as element}
    <article>
        <header>
            <RubyRender string={element.ruby} />
            <span>
                {#each element.inflectionRules as rule}
                    <span class="rule">{rule}</span>
                {/each}
            </span>
        </header>
        <ol>
            {#each element.glossary as def}
                <li>{def}</li>
            {/each}
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
    li {
        white-space: pre-line;
    }
</style>
