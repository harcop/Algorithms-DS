function depositProfit(d: number, r: number, t: number): number {
    let _b = d;
    let i = 0;
    while (_b <= t) {
        _b = _b + (_b * (r/100));
        i++;
    }
    console.log(_b);
    return i;
}

console.log(depositProfit(100, 20, 170))