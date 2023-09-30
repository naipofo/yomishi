<script lang="ts">
    export let check: boolean | null;
</script>

{#if check === null}
    <input type="checkbox" disabled />
{:else}
    <input type="checkbox" bind:checked={check} />
{/if}

<style lang="scss">
    input {
        width: 16px;
        height: 16px;
    }
</style>
