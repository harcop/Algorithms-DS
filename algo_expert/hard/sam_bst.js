const b1 = [10,15,8,12,94,81,5,2,11];
const b2 = [10,8,5,15,2,12,11,94,81];

function sb () {
    return rs(b1, b2, "low") && rs(b1, b2, "high");
}

function rs (a,b,t) {
    if (a.length !== b.length) {
        return false;
    }
    if (a[0] !== b[0]) {
        return false;
    }
    if (a.length <= 1) {
        return true;
    }
    const head = a[0];
    let _a = [];
    let _b = [];
    for(let i=1; i<a.length; i++) {
        if (t === "low") {
            if (a[i] < head) {
                _a.push(a[i]);
                console.log(_a)
            }
            if (b[i] < head) {
                _b.push(b[i]);
            }
            continue;
        }
        if (a[i] > head) {
            _a.push(a[i]);
        }
        if (b[i] > head) {
            _b.push(b[i]);
        }
    }
    let rr = rs(_a, _b, t);
    console.log(_a);
    return rr;
}

console.log(sb());