<script lang="ts">
    import { ApiStore } from "../config-store";
    export let value: ApiStore<boolean>;

    function input(e: { currentTarget: HTMLInputElement }) {
        value.set(e.currentTarget.checked);
    }
</script>

{#if $value.busy}
    <input type="checkbox" checked={$value.value} disabled />
{:else}
    <input type="checkbox" checked={$value.value} on:input={(e) => input(e)} />
{/if}

<style lang="scss">
    input {
        width: 16px;
        height: 16px;
    }
</style>
