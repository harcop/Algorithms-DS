function sumAllPrimes(n: number): number {
    let _n = 5;
    for(let i = 5; i<= n; i++) {
        if (i%2 !== 0 && i%3 !== 0) {
            _n += i;
        }
    }
    return _n
}

console.log(sumAllPrimes(10));
console.log(sumAllPrimes(977));