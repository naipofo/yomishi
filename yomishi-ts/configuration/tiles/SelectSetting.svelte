<script lang="ts">
    import { ApiStore } from "../config-store";

    export let value: ApiStore<string>;
    export let choices: string[];

    function input(e: { currentTarget: HTMLSelectElement }) {
        value.set(e.currentTarget.value);
    }
</script>

<select disabled={$value.busy} on:input={input}>
    {#each choices as choice}
        <option value={choice} selected={choice == $value.value}>
            {choice}
        </option>
    {/each}
</select>

<style>
    select {
        max-width: 300px;
    }
</style>
