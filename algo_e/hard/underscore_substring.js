const str = "testthis is a testtest to see if testestest it works";
const k = "test";

//time O(n) and space O(1);
function us() {
    let _f = '';
    let _c = false;
    let _u = true;
    for(let i = 0; i < str.length; i++) {
        let ele = str[i];
        if (ele === 't') {
            if (str.substr(i,4) === k) {
                if (_f[_f.length-1] === '_') {
                    _f = _f.substring(0, _f.length-1);
                    _u = false;
                }
                if (_u) {
                    _f += '_';
                }
                _f += 'tes';
                _c = true;
                i += 2;
                _u = false;
            }
            else if (_c) {
                _f += 't_';
                _c = false;
                _u = true;
            }else {
                _f += ele;
            }
        }else {
            _f += ele;
        }
    }
    return _f;
};

console.log(us());