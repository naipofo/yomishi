export async function clipboardText() {
    // TODO: work in firefox
    const target = document.querySelector("#copy-target") as HTMLTextAreaElement;
    target.focus();
    document.execCommand("paste");
    return target.value;
}
