<script lang="ts">
    import { ApiStore } from "../config-store";
    import NumberSetting from "./NumberSetting.svelte";

    export let x: ApiStore<number>;
    export let y: ApiStore<number>;

    export let size: boolean = false;

    export let min: number | undefined;
    export let max: number | undefined;
</script>

<div>
    <div>
        <span>
            {#if size}width{:else}x{/if}
        </span>
        <NumberSetting {max} {min} value={x} />
    </div>
    <div>
        <span>
            {#if size}height{:else}y{/if}
        </span>
        <NumberSetting {max} {min} value={y} />
    </div>
</div>

<style lang="scss">
    div {
        display: flex;
        div {
            flex-direction: column;
            &:first-child {
                margin-right: 8px;
            }
        }
        :global(input) {
            width: 80px;
        }
    }
    span {
        font-size: 0.85rem;
    }
</style>
