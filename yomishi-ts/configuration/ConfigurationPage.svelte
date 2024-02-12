<script lang="ts">
    import { createLocalServerTransport } from "../rpc/grcp/transport";
    import ConfigScaffold from "./ConfigScaffold.svelte";
    import RemoteSections from "./RemoteSections.svelte";
    import Section from "./Section.svelte";
    import { createStoreGenerator } from "./config-store";
    import { localConfigEngine, localKeys } from "./engines/local-storage";
    import SettingTile from "./tiles/SettingTile.svelte";
    import TextSetting from "./tiles/TextSetting.svelte";

    const config = createStoreGenerator(localConfigEngine);

    let transport = localConfigEngine
        .get(localKeys.localServerAddress)
        .then((e) => createLocalServerTransport(e));

    config(localKeys.localServerAddress).subscribe((v) => {
        transport = new Promise((r) => r(createLocalServerTransport(v.value)));
    });
</script>

<ConfigScaffold>
    <Section title="Local Server">
        <SettingTile
            title="Local server address"
            desc="URL used to connect to the local yomishi server."
        >
            <TextSetting value={config(localKeys.localServerAddress)} />
        </SettingTile>
    </Section>
    {#await transport then transport}
        <RemoteSections {transport} />
    {/await}
</ConfigScaffold>

<style lang="scss">
    :global(body) {
        background-color: #f8f9fa;
    }
</style>
