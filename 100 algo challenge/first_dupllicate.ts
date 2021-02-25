function firstDuplicate(n: number[]): number {
    let _h = {};
    let _d = n.length;
    let el = -1;
    let change = false;
    
    for(let i = 0; i< n.length; i++) {
        let _di = _h[n[i]];
        if (_di) {
             change = true;
             let _dd = Math.abs(i+1-_di);
             if (_dd < _d) {
                 _d = _dd;
                 el = n[i];
             }
        }
        _h[n[i]] = i+1;
    }

    if (change) {
         return el;
    }
    return -1;
 }

console.log(firstDuplicate([2, 1, 3, 5, 3, 2]));
console.log(firstDuplicate([2, 4, 3, 5, 1]));
