function firstDigit(str: string): string {
    let _s = str.split("");
    let _n = [0,1,2,3,4,5,6,7,8,9];
    for (let i = 0; i < _s.length; i++) {
        let ele = _s[i];
        if (_n.includes(parseInt(ele))) {
            return _s[i];
        }
    };
}

console.log(firstDigit('var_1__Int'));
console.log(firstDigit('q2q-q'));
console.log(firstDigit('0ss'));
