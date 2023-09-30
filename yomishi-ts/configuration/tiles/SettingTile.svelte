<script lang="ts">
    export let title: String;
    export let desc: String;
</script>

<div>
    <header>
        <h1>{title}</h1>
        <span>{desc}</span>
    </header>

    <slot />
</div>

<style lang="scss">
    header {
        flex-grow: 1;
    }
</style>
