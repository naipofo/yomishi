<script lang="ts">
    import { ApiStore } from "../config-store";
    import SettingElement from "./SettingElement.svelte";

    export let value: ApiStore<string[]>;
    export let labelList: [string, string][];

    // store access for components
    $: disabled = $value.busy;
    $: includes = labelList.map(([id]) => $value.value.includes(id));

    const input = (id: string) => (e: { currentTarget: HTMLInputElement }) =>
        value.set(
            !e.currentTarget.checked
                ? [...$value.value, id]
                : $value.value.filter((e) => e != id),
        );
</script>

{#each labelList as [id, label], i}
    <SettingElement title={`${i + 1}. ${label}`}>
        <input type="checkbox" {disabled} on:input={input(id)} checked={!includes[i]} />
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
