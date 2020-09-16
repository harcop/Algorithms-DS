function ratingThreshold(t: number, r: number[][]): number[] {
    let _c = [];
    r.forEach((_r, index) => {
        let _s = 0;
        _r.forEach(ele  => {
            _s += ele;
        })
        let _t = _s/_r.length;
        if (_t < t) {
            _c.push(index);
        }
    })
    return _c;
}

console.log(ratingThreshold(3.5, 
    [[3, 4],
    [3, 3, 3, 4],
    [4]]));