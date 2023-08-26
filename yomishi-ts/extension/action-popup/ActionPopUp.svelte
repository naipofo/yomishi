<script lang="ts">
    const working = "working";
    $: emoji = working == "working" ? "🎉" : "💔";
</script>

<div>
    <h1>Yomishi is {working}! {emoji}</h1>
</div>

<style>
    h1 {
        text-decoration: underline;
    }
</style>
