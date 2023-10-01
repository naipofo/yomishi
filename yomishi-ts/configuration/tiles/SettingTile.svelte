<script lang="ts">
    export let title: String;
    export let desc: String;

    export let wide: boolean = false;
</script>

<div class:wide>
    <header>
        <h1>{title}</h1>
        <span>{desc}</span>
    </header>

    <slot />
</div>

<style lang="scss">
    header {
        flex-grow: 1;
        * {
            font-size: 1rem;
        }
        h1 {
            margin: 0;
            font-weight: bold;
        }
        span {
            font-weight: normal;
            color: #222;
        }
    }

    div {
        display: flex;
        align-items: center;
        padding: 1rem;
        &.wide {
            flex-direction: column;
            align-items: stretch;
            gap: 16px;
        }
        &:not(:last-child) {
            border-bottom: 1px solid #cccccc;
        }
    }
</style>
