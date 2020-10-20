const str1 = "AKICKSTARTPROBLEMNAMEDKICKSTART";
const str2 = "STARTUNLOCKYKICK";
const str3 = "KICKXKICKXSTARTXKICKXSTART";

function ks (str) {
    let _s = str;
    let _m = '';
    let _k = 0;
    let _f = 0;
    let s1 = "KICK"
    let s2 = "START"
    for(let i = 0; i<_s.length; i++) {
        const ele = _s[i];
        if (ele === "K") {
            let _t = str.substr(i, 4);
            if (_t === s1) {
                _k += 1;
                i += 3;
            }
        }
        else if (ele === "S") {
            let _t = _s.substr(i, 5);
            if (_t === s2) {
                _f += _k;
                i += 4;
            }
        }
    }
    return _f;
}

console.log(ks(str1))
console.log(ks(str2))
console.log(ks(str3))