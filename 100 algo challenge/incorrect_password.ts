function incorrectPasscodeAttempts(pc: string, at: string[]): boolean {
    let _t = 0;
    let falser = false;
    for(let i=0; i< at.length; i++) {
        if (_t === 10) {
            falser = true;
            return true;
        }
        _t++;
        if (at[i] === pc) {
            _t = 0;
        }
    }
    return falser;
}

console.log(incorrectPasscodeAttempts('1111', ["1111", "4444", "9999", "3333", "8888", "2222", "7777", "0000", "6666", "7285", "2342", "1111"]));