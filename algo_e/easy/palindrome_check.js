const str = 'abcdcba';

// time O(n) && space O(1);
function pc(str) {
    let _s = str.split("");
    let lp = 0;
    let rp = _s.length-1;
    let i = 0;
    while(lp<rp) {
        if (_s[lp] !== _s[rp]) {
            return false
        }
        lp++;
        rp--;
    }
    return true;
}

// time O(n) && space O(n);
function pc2(str) {
    let _s = str.split("");
    let ns = '';
    for (let i=_s.length-1; i>=0; i--) {
        console.log(i);
        ns += _s[i];
    }
    console.log(ns);
    if (str === ns) {
        return true;
    }
    return false;
}

console.log(pc2(str));