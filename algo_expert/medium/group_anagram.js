const arr = ["go", "act", "flop", "tac", "cat", "og", "olfp"];

function ga (arr) {
    let ht = {};
    let _f = [];
    arr.forEach((ele, index) => {
        let _e = ele.split("");
        _e.sort();
        _e = _e.join('');
        if (ht[_e] === undefined) {
            ht[_e] = [];
        }
        ht[_e].push(ele);
    })
    for (const _h in ht) {
        const _hh = ht[_h];
        const _ff = [];
        _hh.forEach(ele => {
            _ff.push(ele);
        })
        _f.push(_ff);
    }
    return _f;
}
console.log(ga(arr));