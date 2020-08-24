function lateRide(n: number): number {
    let _m = Math.floor(n/60);
    let _r = n - (_m*60);
    let _jmr = `${_m}${_r}`.split("");
    let _j = 0;
    _jmr.forEach(ele => {
        _j += parseInt(ele);
    });
    return _j;
}

console.log(lateRide(240));
console.log(lateRide(808));
