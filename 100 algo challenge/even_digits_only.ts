function evenDigitsOnly(n: number): boolean {
    let _n = n.toString().split("");
    for(let i =0; i<_n.length; i++) {
        let ele = _n[i];
        if (parseInt(ele)%2 !== 0) {
            return false;
        }   
    };
    return true;
}

console.log(evenDigitsOnly(248622));
console.log(evenDigitsOnly(642386));