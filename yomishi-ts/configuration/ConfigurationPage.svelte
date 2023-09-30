<script>
    import { createLocalServerTransport } from "../rpc/transport";
    import ConfigScaffold from "./ConfigScaffold.svelte";
    import Section from "./Section.svelte";
    import { createConfigStoreProvider } from "./config-store";
    import SettingTile from "./tiles/SettingTile.svelte";
    import ToggleSetting from "./tiles/ToggleSetting.svelte";

    let provider = createConfigStoreProvider(
        createLocalServerTransport("http://127.0.0.1:50051"),
    );

    let store = provider.createBoolStore("AnkiEnabled");
</script>

<ConfigScaffold>
    <Section title="Dictionaries" />
    <Section title="Anki">
        <SettingTile
            title="Use AnkiConnect"
            desc="Use AnkiConnect API to create flashcards."
        >
            <ToggleSetting bind:check={$store} />
        </SettingTile>
    </Section>
    <Section title="Popup" />
</ConfigScaffold>
