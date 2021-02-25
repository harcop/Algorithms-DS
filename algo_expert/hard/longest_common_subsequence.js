const str1 = ['z','x','v','v','y','z','w','k'];
const str2 = ['x','k','y','k','z','p','w','k'];

//time O(n^2) && space O(n)
function lcs1 () {
    let ht = {};
    str1.forEach((ele, idx) => {
        let _m = ''
        if (str2.includes(ele)) {
            for (let t in ht) {
                const { index, value } = ht[t];
                if (str2.indexOf(t) < str2.indexOf(ele)) {
                    if (ht[t].length > _m.length) {
                        _m = ht[t];
                    } 
                }
            }
            ht[ele] = {
                value: `${_m}${ele}`,
                index: idx
            }
        }
    })
    let _n = '';
    for(let k in ht) {
        if (ht[k].length > _n.length) {
            _n = ht[k];
        }
    }
    return ht;
}


function lcs (a,b) {
    let _k = [];
    a.unshift(" ");
    b.unshift(" ");
    for (let i=0; i<a.length;i++) {
        _k.push([]);
        _k[i][0] = "!";
        _k[0].push("!");
    }
    console.log(_k[0])
    for(let i=1; i<a.length; i++) {
        for (let j=1; j<b.length; j++) {
            let _a = _k[i-1][j];
            let _b = _k[i][j-1];
            if (a[i] !== b[j]) {
                if (_a.length > _b.length) {
                    _k[i][j] = _a;
                }else {
                    _k[i][j] = _b;
                }
            } else {
                if (_a.length > _b.length) {
                    _k[i][j] =  _a+a[i];
                }else {
                    _k[i][j] = _b+a[i];
                }
            }
        }
    }
    return _k[_k.length-1][_k[_k.length-1].length-1];
}

console.log(lcs(str1, str2));