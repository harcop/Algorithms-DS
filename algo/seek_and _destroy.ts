function seekAndDestroy(a1: number[], a2: number[]): number[] {
    let _f = [];
    a1.forEach(ele => {
        if (!a2.includes(ele)) {
            _f.push(ele);
        }
    })
    return _f;
}

console.log(seekAndDestroy([3, 5, 1, 2, 2], [2, 3, 5]));
console.log(seekAndDestroy([1, 2, 3, 5, 1, 2, 3], [2, 3]));
