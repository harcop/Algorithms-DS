function isLucky(n: number): boolean {
    let _l = Math.floor(n.toString().length/2);
    let sum1 = 0;
    let sum2 = 0;
    let fp = 0
    let rp = _l;
    for (let i = 0; i <_l; i++) {
        sum1 += parseInt(n.toString()[fp++])
        sum2 += parseInt(n.toString()[rp++])
    }
    if (sum1 === sum2) {
        return true;
    }
    return false;
}

console.log(isLucky(1230));
console.log(isLucky(1533));
console.log(isLucky(239017));