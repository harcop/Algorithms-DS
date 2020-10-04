function launchSequenceChecker(sys: string[], sn: number[]): boolean {
    let _a = {};
    sys.forEach((_s, index) => {
        if (typeof _a[_s] === "undefined") {
            _a[_s] = [];
        }
        _a[_s].push(sn[index]);
    })
    for(const _s in _a) {
        let _l = _a[_s];
        for(let i = 1; i<_l.length; i++) {
            if (_l[i] < _l[i-1]) {
                return false;
            }
        }
    }
    return true;
}

console.log(launchSequenceChecker(["stage_1", "stage_2", "dragon", "stage_1", "stage_2", "dragon"], [1, 10, 11, 2, 12, 111]));
console.log(launchSequenceChecker(["stage_1", "stage_1", "stage_2", "dragon"], [2, 1, 12, 111]));