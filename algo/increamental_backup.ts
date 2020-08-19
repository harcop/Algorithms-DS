function incrementalBackups(lbt: number, ch: number[][]): number[] {
    let _n = [];
    ch.forEach(ele => {
        if (ele[0] > lbt) {
            if (!_n.includes(ele[1])){
                _n.push(ele[1]);
            }
        }
    });

    return _n.sort((a, b) => a-b);
}

console.log(incrementalBackups(461620205, [[461620203, 1], 
    [461620204, 2], 
    [461620205, 6],
    [461620206, 5], 
    [461620207, 3], 
    [461620207, 5], 
    [461620208, 1]]));
