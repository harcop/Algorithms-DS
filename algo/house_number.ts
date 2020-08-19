function houseNumbersSum(a: number[]): number {
    let _s = 0;
    let i = 0;
    while (a[i] !== 0) {
        _s += a[i];
        i++;
    }
    return _s
}

console.log(houseNumbersSum([5, 1, 2, 3, 0, 1, 5, 0, 2]));