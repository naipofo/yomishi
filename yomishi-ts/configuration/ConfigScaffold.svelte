<script lang="ts">
    import { ScaffoldContext } from "./scaffold-context";

    let sections: string[] = [];

    new (class extends ScaffoldContext {
        registerSection(name: string): void {
            sections = [...sections, name];
        }
    })().setCurrentContext();
</script>

<div>
    <nav>
        <!-- TODO: jump to section links -->
        {#each sections as section}
            <p>{section}</p>
        {/each}
    </nav>
    <main>
        <slot />
    </main>
</div>

<style lang="scss">
    div {
        display: flex;
    }
    main {
        flex-grow: 1;
        display: flex;
        flex-direction: column;
        align-items: center;
        gap: 16px;
    }
    nav {
        display: none;
    }
</style>
