function differentSymbolsNaive(s: string): number {
    let _s = s.split("");
    let _sym = [];
    _s.forEach(ele => {
        if (_sym.includes(ele)) {
            return;
        }
        _sym.push(ele);
    })
    return _sym.length;
}

console.log(differentSymbolsNaive('cabca'));
