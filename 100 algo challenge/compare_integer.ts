function compareIntegers(a: string, b: string): string {
    let [_a, _b] = [parseInt(a), parseInt(b)];
    if (_a === _b) {
        return 'equal';
    }
    else if (_a > _b) {
        return 'greater'
    }
    return 'less';
}

console.log(compareIntegers('12', '13'));
console.log(compareIntegers('875', '799'));
console.log(compareIntegers('1000', '1000'));