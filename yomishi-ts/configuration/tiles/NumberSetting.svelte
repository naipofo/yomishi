<script lang="ts">
    import { ApiStore } from "../config-store";
    import { debounceToStore } from "./store-debounce";

    export let value: ApiStore<number>;

    const input = debounceToStore<number>(value, (e) => {
        const r = parseInt(e);
        return r > 0 ? r : 0;
    });
</script>

<input type="number" value={$value.value} class:busy={$value.busy} on:input={input} />

<style lang="scss">
    .busy {
        background-color: gray;
    }
</style>
