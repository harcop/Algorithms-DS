function sumOfTwo(a: number[], b: number[], v: number): boolean {
    for (let ele of a) {
        let _r = v - ele;
        if (b.includes(_r)) {
            return true;
        }
    }
    return false;
}

console.log(sumOfTwo([1, 2, 3], [10, 20, 30, 40], 42));