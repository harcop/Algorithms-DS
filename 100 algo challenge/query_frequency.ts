function frequency (k: number, arr: number[][]): number[] {
    let _oa = []; // arr to be tracked in loop;
    let _f = []; // frequency of the query
    let _hashF = {}; //hash of every element

    for (let i=0; i<k; i++) {
        const _a = arr[i];
        let [q1, q2] = [_a[0], _a[1]]; 
        if (q1 === 1) {
            _oa.push(q2);
            if (typeof _hashF[q2] === 'undefined') {
                _hashF[q2] = 0
            }
            _hashF[q2] += 1;
        }
        else if (q1 === 2) {
            if (_oa.includes(q2)) {
                _oa.splice(_oa.indexOf(q2), 1);
                _hashF[q2] -= 1;
            }
        }
        else if (q1 === 3) {
            let found = false;
            for(let _hash in _hashF) {
                if (_hashF[_hash] === q2) {
                    found = true;
                    break;
                }
            }
            if (found) {
                _f.push(1);
            }else {
                _f.push(0);
            }
        }
    }
    return _f
}

console.log(frequency(7, [[1,1], [2,2], [3,2], [1,1], [1,1], [2,1], [3,2]]))
console.log(frequency(4, [[3,4], [2,1003], [1,16], [3,1]]))
console.log(frequency(10, [[1,3], [2,3], [3,2], [1,4], [1,5],[1,5], [1,4], [3,2], [2,4], [3,2]]))