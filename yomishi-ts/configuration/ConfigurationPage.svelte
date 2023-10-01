<script lang="ts">
    import { Config } from "@yomishi-proto/config_connect";
    import { createGenericRpcClient } from "../rpc/generic-client";
    import { createLocalServerTransport } from "../rpc/transport";
    import ConfigScaffold from "./ConfigScaffold.svelte";
    import Section from "./Section.svelte";
    import { createConfigStoreProvider } from "./config-store";
    import DimensionsSetting from "./tiles/DimensionsSetting.svelte";
    import SettingTile from "./tiles/SettingTile.svelte";
    import TextSetting from "./tiles/TextSetting.svelte";
    import ToggleListSetting from "./tiles/DictListSetting.svelte";
    import ToggleSetting from "./tiles/ToggleSetting.svelte";
    import { DictionaryListRequest } from "@yomishi-proto/config_pb";

    const transport = createLocalServerTransport("http://127.0.0.1:50051");

    const config = createConfigStoreProvider(transport);

    const configClient = createGenericRpcClient(transport, Config);
    const dictList = configClient
        .dictionaryList(DictionaryListRequest.fromJson({}))
        .then((r) =>
            r.dictionaries
                .map((e) => [Number(e.id), e.name] as [number, string])
                .sort((a, b) => a[0] - b[0]),
        );

    (async () => {
        console.log(
            await configClient.dictionaryList(DictionaryListRequest.fromJson({})),
        );
    })();
</script>

<ConfigScaffold>
    <Section title="Dictionaries">
        <SettingTile
            title="Toggle Enabled Dictionaries"
            desc="Enable or disable dictionaries from search"
            wide={true}
        >
            {#await dictList}
                <p>loading...</p>
            {:then labelList}
                <ToggleListSetting value={config("DictionariesDisabled")} {labelList} />
            {/await}
        </SettingTile>
    </Section>
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
