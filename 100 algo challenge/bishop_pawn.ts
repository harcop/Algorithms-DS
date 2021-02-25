function bishopAndPawn(b: string, p: string): boolean {
    let _b = b.split("");
    let _p = p.split("");
    let _charDiff = Math.abs(_b[0].charCodeAt(0) - _p[0].charCodeAt(0));
    let _numDiff = Math.abs(parseInt(_b[1]) - parseInt(_p[1]));
    if (_charDiff !== _numDiff) {
        return false
    }
    return true;
}

console.log(bishopAndPawn('a1', 'c3'));
console.log(bishopAndPawn('c2', 'd3'));