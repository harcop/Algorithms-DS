const str = 'xyz';

// time O(n) && space O(n);
function cce(str, k) {
    let ns = '';
    for (let i=0;i<str.length;i++) {
        let nlc = str.charCodeAt(i) + k;
        if (nlc > 122) {
            nlc = 96 + (nlc%122)
        }
        ns += String.fromCharCode(nlc);
    }
    return ns;
}

console.log(cce(str, 3));