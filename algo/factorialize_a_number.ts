function factorializeANumber(num: number): number {
    if (num <= 1) {
        return 1;
    }
    let i = 2;
    let n = 1;
    while(i<=num) {
        n *= i;
        i++;
    }
    return n
}

console.log(factorializeANumber(5));
console.log(factorializeANumber(10));