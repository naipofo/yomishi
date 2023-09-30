<script>
    import { createLocalServerTransport } from "../rpc/transport";
    import ConfigScaffold from "./ConfigScaffold.svelte";
    import Section from "./Section.svelte";
    import { createConfigStoreProvider } from "./config-store";
    import NumberSetting from "./tiles/NumberSetting.svelte";
    import SettingTile from "./tiles/SettingTile.svelte";
    import TextSetting from "./tiles/TextSetting.svelte";
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
        <SettingTile
            title="AnkiConnect server address"
            desc="URL used to connect to AnkiConnect."
        >
            <TextSetting value={config.string("AnkiConnectAddress")} />
        </SettingTile>
    </Section>
    <Section title="Popup">
        <!-- TODO: double-value input for popup size -->
        <SettingTile title="Popup Width" desc="Width of the dictionary popup in pixels">
            <NumberSetting value={config.number("PopupWidth")} />
        </SettingTile>
        <SettingTile
            title="Popup Height"
            desc="Height of the dictionary popup in pixels"
        >
            <NumberSetting value={config.number("PopupHeight")} />
        </SettingTile>
    </Section>
</ConfigScaffold>

<style lang="scss">
    :global(body) {
        background-color: #f8f9fa;
    }
</style>
