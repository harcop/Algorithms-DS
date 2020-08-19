function growingPlant(us: number, ds: number, dh: number): number {
    let _d = 0;
    for(let i = 100; i <= dh; i = i+us) {
        _d++;
        i -= ds;
    }
    return _d;
}

console.log(growingPlant(100, 10, 910))