<script>
    import { createLocalServerTransport } from "../rpc/transport";
    import ConfigScaffold from "./ConfigScaffold.svelte";
    import Section from "./Section.svelte";
    import { createConfigStoreProvider } from "./config-store";
    import SettingTile from "./tiles/SettingTile.svelte";
    import ToggleSetting from "./tiles/ToggleSetting.svelte";

    let config = createConfigStoreProvider(
        createLocalServerTransport("http://127.0.0.1:50051"),
    );
</script>

<ConfigScaffold>
    <Section title="Dictionaries" />
    <Section title="Anki">
        <SettingTile
            title="Use AnkiConnect"
            desc="Use AnkiConnect API to create flashcards."
        >
            <ToggleSetting value={config.boolean("AnkiEnabled")} />
        </SettingTile>
    </Section>
    <Section title="Popup" />
</ConfigScaffold>
