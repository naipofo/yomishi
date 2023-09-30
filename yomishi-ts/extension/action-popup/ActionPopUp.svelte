<script lang="ts">
    import { browser } from "../browser-extension";

    const working = "working";
    $: emoji = working == "working" ? "🎉" : "💔";
</script>

<div>
    <h1>Yomishi is {working}! {emoji}</h1>
    <button on:click={() => browser.runtime.openOptionsPage()}>Open settings</button>
</div>

<style>
    h1 {
        text-decoration: underline;
    }
</style>
