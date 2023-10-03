<script lang="ts">
    import { ApiStore } from "../config-store";

    export let value: ApiStore<string>;
    export let choices: string[];

    function input(e: { currentTarget: HTMLSelectElement }) {
        value.set(e.currentTarget.value);
    }
</script>

<select disabled={$value.busy} on:input={input} value={$value.value}>
    {#each choices as choice}
        <option value={choice}>{choice}</option>
    {/each}
</select>
