function largestOfFour(n: number[][]): number[] {
    let _ma = [];
    n.forEach(e => {
        let _m = 0;
        e.forEach(k => {
            _m = max(_m,k);
        })
        _ma.push(_m);
    })
    return _ma;
}

function max(a,b) {
    if (a>b){
        return a;
    }
    return b;
}

console.log(largestOfFour([[4, 5, 1, 3], [13, 27, 18, 26], [32, 35, 37, 39], [1000, 1001, 857, 1]]) );
console.log(largestOfFour([[4, 9, 1, 3], [13, 35, 18, 26], [32, 35, 97, 39], [1000000, 1001, 857, 1]]));