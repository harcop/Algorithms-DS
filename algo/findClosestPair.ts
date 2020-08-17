function findClosestPair(n: number[], s: number): number {
    let _h = {};
    let _d = n.length;
    let change = false;
    for(let i = 0; i< n.length; i++) {
        let _di = _h[nI(n[i])];
        if (_di) {
             change = true;
             let _dd = Math.abs(i+1-_di);
             if (_dd < _d) {
                 _d = _dd;
             }
        }
        _h[n[i]] = i+1;
    }
 
    function nI(e) {
        let c = (s - e);
        return c;
    }
    if (change) {
         return _d;
    }
    return -1;
 }
 
 console.log(findClosestPair([1, 0, 2, 4, 3, 0], 5));
 console.log(findClosestPair([2, 3, 7], 8));