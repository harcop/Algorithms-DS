function confirmEnding(str: string, tg: string) {
    let _l = tg.length;
    let _s = str.substr(_l-1);
    if (_s === tg) {
        return true;
    }
    return false;
}

console.log(confirmEnding("Abstraction", "action"));
console.log(confirmEnding("Open sesame", "pen"));