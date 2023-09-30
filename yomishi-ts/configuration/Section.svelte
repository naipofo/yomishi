<script lang="ts">
    import { ScaffoldContext } from "./scaffold-context";

    export let title: string;

    ScaffoldContext.getCurrentContext()?.registerSection(title);
</script>

<section>
    <header>
        {title}
    </header>
    <main>
        <slot>no settings</slot>
    </main>
</section>

<style lang="scss">
    section {
        width: min(800px, calc(100vw - 40px));
    }
    main {
        background-color: white;
    }
    header {
        font-size: 1.4rem;
        padding-left: 1rem;
    }
</style>
