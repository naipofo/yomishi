<script>
    import { createLocalServerTransport } from "../rpc/transport";
    import ConfigScaffold from "./ConfigScaffold.svelte";
    import Section from "./Section.svelte";
    import { createConfigStoreProvider } from "./config-store";
    import DimensionsSetting from "./tiles/DimensionsSetting.svelte";
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
            <ToggleSetting value={config("AnkiEnabled")} />
        </SettingTile>
        <SettingTile
            title="AnkiConnect server address"
            desc="URL used to connect to AnkiConnect."
        >
            <TextSetting value={config("AnkiConnectAddress")} />
        </SettingTile>
    </Section>
    <Section title="Popup">
        <SettingTile title="Popup Size" desc="Size of the dictionary popup in pixels">
            <DimensionsSetting
                x={config("PopupHeight")}
                y={config("PopupWidth")}
                min={100}
                max={10000}
                size={true}
            />
        </SettingTile>
    </Section>
</ConfigScaffold>

<style lang="scss">
    :global(body) {
        background-color: #f8f9fa;
    }
</style>
