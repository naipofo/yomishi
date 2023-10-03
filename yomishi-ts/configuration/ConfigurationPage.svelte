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
    import { localAddress } from "../rpc/address-manager";
    import { createConfigDataStore } from "./config-data-store";
    import SelectSetting from "./tiles/SelectSetting.svelte";
    import FieldMapSetting from "./tiles/FieldMapSetting.svelte";

    const transport = createLocalServerTransport(localAddress);

    const config = createConfigStoreProvider(transport);

    const configClient = createGenericRpcClient(transport, Config);
    const dictList = configClient
        .dictionaryList(DictionaryListRequest.fromJson({}))
        .then((r) =>
            r.dictionaries
                .map((e) => [Number(e.id), e.name] as [number, string])
                .sort((a, b) => a[0] - b[0]),
        );

    const configData = createConfigDataStore(transport);
    config("AnkiModelName").subscribe((e) => e.busy || configData.refresh());
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
        <SettingTile title="Anki Deck name" desc="Deck in which notes will be created.">
            <SelectSetting value={config("AnkiDeckName")} choices={$configData.decks} />
        </SettingTile>
        <SettingTile
            title="Anki Tag"
            desc="Tag added to created noted. Leave empty to not add any."
        >
            <TextSetting value={config("AnkiTag")} />
        </SettingTile>
        <SettingTile title="Anki Note Type" desc="Note type used to create notes.">
            <SelectSetting
                value={config("AnkiModelName")}
                choices={$configData.models}
            />
        </SettingTile>
        <SettingTile
            title="Anki Note Format"
            desc="Field mapping for generated notes."
            wide={true}
        >
            <FieldMapSetting
                value={config("AnkiFields")}
                fields={$configData.currentModelFields}
            />
        </SettingTile>
    </Section>
    <Section title="Popup">
        <SettingTile title="Popup Size" desc="Size of the dictionary popup in pixels.">
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
