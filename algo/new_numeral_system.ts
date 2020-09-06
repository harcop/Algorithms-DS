function newNumeralSystem(n: string): string[] {
    let _n = n.charCodeAt(0);
    let _a = 65;
    let _f = [];
    while (_n >= _a) {
        let s = `${String.fromCharCode(_a)} + ${String.fromCharCode(_n)}`;
        _a++;
        _n--;
        _f.push(s);
    }
    return _f;
}   

console.log(newNumeralSystem('G'));