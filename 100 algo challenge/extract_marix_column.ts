function extractMatrixColumn(m: number[][], c: number): number[] {
    let i = [];
    m.forEach(ele => {
        i.push(ele[c])
    })
    return i;
}

console.log(extractMatrixColumn([[1, 1, 1, 2], [0, 5, 0, 4], [2, 1, 3, 6]], 2));