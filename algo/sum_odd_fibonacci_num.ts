function sumOddFibonacciNums(num: number): number {
    let n = 2;
    let _n = 2;
    let b = [1,1];
    while (n <= num) {
        let _t = b[0] + b[1];
        b[0] = b[1];
        b[1] = _t;
        if (b[1] % 2 === 1) {
            _n += b[1];
        }
        n += b[1];
    }
    return _n;
}

console.log(sumOddFibonacciNums(1000));
console.log(sumOddFibonacciNums(4000000));
console.log(sumOddFibonacciNums(10));