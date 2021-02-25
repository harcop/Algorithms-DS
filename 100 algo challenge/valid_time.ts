function validTime(time: string): boolean {
    let _t = time.split(":");
    if (_t.length === 0) {
        return false;
    }
    if (_t[0] < "00" || _t[0] > "23") {
        return false;
    }
    if (_t[1] < "00" || _t[1] > "59") {
        return false;
    }
    return true;
}

console.log(validTime('13:58'));
console.log(validTime('25:51'));
console.log(validTime('02:76'));