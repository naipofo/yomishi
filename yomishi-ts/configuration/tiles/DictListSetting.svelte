<script lang="ts">
    import { ApiStore } from "../config-store";
    import SettingElement from "./SettingElement.svelte";

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

{#each labelList as [id, label], i}
    <SettingElement title={`${id}. ${label}`}>
        <input type="checkbox" {disabled} on:input={input(id)} checked={includes[i]} />
    </SettingElement>
{/each}
{#if labelList.length == 0}
    <p>no dictionaries</p>
{/if}

<style lang="scss">
    input {
        width: 16px;
        height: 16px;
    }
</style>
