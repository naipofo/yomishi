chrome.runtime.onMessage.addListener((_, _1, respond) => {
    const target = document.querySelector("#copy-target") as HTMLTextAreaElement;
    target.focus();
    document.execCommand("paste");
    respond(target.value);
});
