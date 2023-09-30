<script lang="ts">
    import { ScaffoldContext } from "./scaffold-context";

    export let title: string;

    ScaffoldContext.getCurrentContext()?.registerSection(title);
</script>

<section>
    <h2>{title}</h2>
    <main>
        <slot>no settings</slot>
    </main>
</section>
