console.log("yomishi init!");
document.body.addEventListener("mousemove", (e) => {
    const hoveredString = getStringFromCaret(e.clientX, e.clientY);
    console.log(hoveredString);
});

function getStringFromCaret(x: number, y: number): string {
    const anyDocument = document as any;
    let range: any;
    let offset: number;
    let textNode;

    if (typeof anyDocument.caretRangeFromPoint === "function") {
        // Chromium
        range = anyDocument.caretRangeFromPoint(x, y);
        offset = range.startOffset;
        textNode = range.startContainer;
    } else if (typeof anyDocument.caretPositionFromPoint === "function") {
        // Firefox
        range = anyDocument.caretPositionFromPoint(x, y);
        offset = range.offset;
        textNode = range.offsetNode;
    } else {
        throw new Error("carret support not found!");
    }
    let data = textNode.textContent as string;
    // TODO: break node boudaries
    return data.substring(offset, data.length);
}
