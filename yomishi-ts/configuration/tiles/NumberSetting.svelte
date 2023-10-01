<script lang="ts">
    import { ApiStore } from "../config-store";
    import { debounceToStore, numberTransform } from "./store-debounce";

    export let value: ApiStore<number>;

    export let min: number | undefined;
    export let max: number | undefined;

    const input = debounceToStore<number>(value, numberTransform(min, max));
</script>

<input type="number" value={$value.value} class:busy={$value.busy} on:input={input} />

<style lang="scss">
    .busy {
        background-color: gray;
    }
</style>
