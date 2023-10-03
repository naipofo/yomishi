<script lang="ts">
    import { ApiStore } from "../config-store";
    import SettingElement from "./SettingElement.svelte";

    export let value: ApiStore<Record<string, string>>;
    export let fields: string[];

    let data: Record<string, string> = {};

    // TODO: figure out something cleaner
    let elements = [] as HTMLInputElement[];

    const input = (_: { currentTarget: HTMLInputElement }) => {
        // TODO: debounce
        value.set(
            Object.fromEntries(
                elements
                    .filter((e) => e && e.dataset.title && e.value)
                    .map((e) => [e.dataset.title!, e.value]),
            ),
        );
    };
</script>

{#each fields as title, i}
    <SettingElement {title}>
        <!-- TODO: suggest existing templates -->
        <input
            type="text"
            value={$value.value[title] || ""}
            on:input={input}
            bind:this={elements[i]}
            data-title={title}
        />
    </SettingElement>
{/each}

<style lang="scss">
</style>
