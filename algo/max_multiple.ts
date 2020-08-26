function maxMultiple(d: number, b: number): number  {
    let _n = b-(b%d);
    return _n;
}

console.log(maxMultiple(3,10)); // 9
console.log(maxMultiple(5,8)); // 6