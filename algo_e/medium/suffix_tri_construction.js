const str = "babc";
function stc() {
    let _s = str.split("");
    const ht = {};
    _s.forEach((ele, index) => {
        if (ht[ele] === undefined) {
            ht[ele] = {};
        }
        let pointer = ht[ele];
        for (let i=index+1; i<_s.length; i++) {
            if (pointer[_s[i]] === undefined) {
                pointer[_s[i]] = {}
            }
            pointer = pointer[_s[i]];
        }
    })
    return ht;
}
console.log(stc())