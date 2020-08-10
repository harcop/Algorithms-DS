function commonCharacterCount(s1: string, s2: string): number {
    let _s1 = s1.split("");
    let _s2 = s2.split("");
    let _c = [];
    _s1.forEach(ele => {
        if (_s2.includes(ele)) {
            _c.push(ele);
            _s2.splice(_s2.indexOf(ele), 1);
        }
    });
    return _c.length;
}

console.log(commonCharacterCount('aabcc', 'adcaa'));