<script lang="ts">
    import { ApiStore } from "../config-store";
    import { debounceToStore } from "./store-debounce";

    export let value: ApiStore<string>;

    const input = debounceToStore(value);
</script>

<input type="text" value={$value.value} class:busy={$value.busy} on:input={input} />

<style lang="scss">
    .busy {
        background-color: gray;
    }
</style>
