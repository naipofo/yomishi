<script lang="ts">
    import { ApiStore } from "../config-store";

    export let value: ApiStore<number[]>;
    export let labelList: [number, string][];

    // store access for components
    $: disabled = $value.busy;
    $: includes = labelList.map(([id]) => $value.value.includes(id));

    const input = (id: number) => (e: { currentTarget: HTMLInputElement }) =>
        value.set(
            e.currentTarget.checked
                ? [...$value.value, id]
                : $value.value.filter((e) => e != id),
        );
</script>

<ol class:busy={$value.busy}>
    {#each labelList as [id, label], i}
        <li>
            <span>
                {id}.
                <i>{label}</i>
            </span>
            <input
                type="checkbox"
                {disabled}
                on:input={input(id)}
                checked={includes[i]}
            />
        </li>
    {/each}
    {#if labelList.length == 0}
        <p>no dictionaries</p>
    {/if}
</ol>

<style lang="scss">
    input {
        width: 16px;
        height: 16px;
    }
    ol {
        padding: 0;
        margin: 0;
    }
    li {
        display: flex;
        font-size: 0.85rem;
    }
    li:not(:last-child) {
        padding-bottom: 8px;
    }
    span {
        flex-grow: 1;
    }
</style>
