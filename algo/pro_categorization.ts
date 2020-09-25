function proCategorization(d: string[], ps: string[][]): string[][][] {
    let _j = {}
    ps.forEach((p, index) => {
        for (const _p of p) {
            if (typeof _j[_p] === 'undefined') {
                _j[_p] = [];
            }
            _j[_p].push(d[index])
        }
    })
    let _f = [];
    for (let _p in _j) {
        let _d = [[_p], _j[_p]]
        _f.push(_d);
    }
    return _f
}

console.log(proCategorization(["Jack", "Leon", "Maria"], [["Computer repair", "Handyman", "House cleaning"],
["Computer lessons", "Computer repair", "Data recovery service"],
["Computer lessons", "House cleaning"]]));