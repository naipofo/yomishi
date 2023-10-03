<script lang="ts">
    export let title: String;
</script>

<div>
    <header>{title}</header>
    <slot />
</div>

<style lang="scss">
    div {
        display: flex;
        align-items: center;
        &:not(:last-child) {
            padding-bottom: 8px;
        }
    }
    header {
        font-size: 0.85rem;
        font-weight: normal;
        color: #222;
        flex-grow: 1;
    }
</style>
