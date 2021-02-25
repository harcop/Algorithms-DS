const pi = "3141592";
const arr = ['3141', '5', '31', '2', '4159', '9', '42'];

function np() {

    let _g = [];
    for(const ele of arr) {
        for (let i = 0; i<pi.length; i++) {
            if (_g[i] === undefined) {
                _g.push([]);
            }
            _g[i].push(0);
        }
    }
    for(let j =0;  j<arr.length; j++) {
        let ele = arr[j];
        let id = parseInt(ele[0]);
        console.log(id);
        let _m = 0;
        for (let i = 0; i<pi.length; i++) {
            if (pi[i] === ele[0]) {
                console.log(pi.substr(i,ele.length))
                if (pi.substr(i,ele.length) === ele) {
                    _g[j][i] = ele;
                }
                break;
            }
        }
    }
    let _f = [];
    for(let j = 0;  j<_g.length; j++) {
        let _m = 0;
        let _s = 0;
        let _j = j;
        for (let i = 0; i<_g[j].length; i++) {
            if(_g[j][i] !== 0) {
                let _l = _g[j][i].length;
                console.log(_g[j][i]);
                _m += _l;
                _s += 1;
                i += _l - 1;
                j += 1;
                console.log(_l);
                // break;
            }
        }
        j = _j;
        _f.push(_s);
    }

    return [_g, _f];
}
console.log(np()); 