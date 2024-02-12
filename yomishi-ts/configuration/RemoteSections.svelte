<script lang="ts">
    import { rpcKeys } from "@yomishi-config/config";
    import { Config } from "@yomishi-proto/config_connect";
    import { DictionaryListRequest } from "@yomishi-proto/config_pb";
    import { createGenericRpcClient } from "../rpc/grcp/generic-client";
    import { RpcTransport } from "../rpc/grcp/transport";
    import Section from "./Section.svelte";
    import { createConfigHelperStore } from "./config-helper-store";
    import { createStoreGenerator } from "./config-store";
    import { createConfigRpcEngine } from "./engines/config-rpc";
    import ToggleListSetting from "./tiles/DictListSetting.svelte";
    import DimensionsSetting from "./tiles/DimensionsSetting.svelte";
    import FieldMapSetting from "./tiles/FieldMapSetting.svelte";
    import SelectSetting from "./tiles/SelectSetting.svelte";
    import SettingTile from "./tiles/SettingTile.svelte";
    import TextSetting from "./tiles/TextSetting.svelte";
    import ToggleSetting from "./tiles/ToggleSetting.svelte";

    export let transport: RpcTransport;

    const config = createStoreGenerator(createConfigRpcEngine(transport));

    const configClient = createGenericRpcClient(transport, Config);
    const dictList = configClient
        .dictionaryList(DictionaryListRequest.fromJson({}))
        .then((r) =>
            r.dictionaries
                .map((e) => [Number(e.id), e.name] as [number, string])
                .sort((a, b) => a[0] - b[0]),
        );

    const configData = createConfigHelperStore(transport);

    config(rpcKeys.AnkiModelName).subscribe((e) => e.busy || configData.refresh());
</script>

<Section title="Dictionaries">
    <SettingTile
        title="Toggle Enabled Dictionaries"
        desc="Enable or disable dictionaries from search"
        wide={true}
    >
        {#await dictList}
            <p>loading...</p>
        {:then labelList}
            <ToggleListSetting
                value={config(rpcKeys.DictionariesDisabled)}
                {labelList}
            />
        {/await}
    </SettingTile>
</Section>
<Section title="Anki">
    <SettingTile
        title="Use AnkiConnect"
        desc="Use AnkiConnect API to create flashcards."
    >
        <ToggleSetting value={config(rpcKeys.AnkiEnabled)} />
    </SettingTile>
    <SettingTile
        title="AnkiConnect server address"
        desc="URL used to connect to AnkiConnect."
    >
        <TextSetting value={config(rpcKeys.AnkiConnectAddress)} />
    </SettingTile>
    <SettingTile title="Anki Deck name" desc="Deck in which notes will be created.">
        <SelectSetting
            value={config(rpcKeys.AnkiDeckName)}
            choices={$configData.decks}
        />
    </SettingTile>
    <SettingTile
        title="Anki Tag"
        desc="Tag added to created noted. Leave empty to not add any."
    >
        <TextSetting value={config(rpcKeys.AnkiTag)} />
    </SettingTile>
    <SettingTile title="Anki Note Type" desc="Note type used to create notes.">
        <SelectSetting
            value={config(rpcKeys.AnkiModelName)}
            choices={$configData.models}
        />
    </SettingTile>
    <SettingTile
        title="Anki Note Format"
        desc="Field mapping for generated notes."
        wide={true}
    >
        <FieldMapSetting
            value={config(rpcKeys.AnkiFields)}
            fields={$configData.currentModelFields}
        />
    </SettingTile>
</Section>
<Section title="Popup">
    <SettingTile title="Popup Size" desc="Size of the dictionary popup in pixels.">
        <DimensionsSetting
            x={config(rpcKeys.PopupWidth)}
            y={config(rpcKeys.PopupHeight)}
            min={100}
            max={10000}
            size={true}
        />
    </SettingTile>
</Section>
