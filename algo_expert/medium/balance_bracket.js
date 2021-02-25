const arr = ["(","(","[","]","(",")",")","{","}",")"];

function bb (arr) {
    let bt = [];
    let ob = ["(","[","{"];
    let cb = [")","]","}"];
    for (let i=0; i<arr.length; i++) {
        let b = arr[i];
        console.log(b)
        if (b === '(' || b === '[' || b === '{') {
            bt.push(b);
            continue;
        } 
        let ib = cb.indexOf(b);
        if (bt[bt.length -1] !== ob[ib]) {
            return false;
        }
        bt.pop();
    }
    if (bt.length > 0) {
        return false;
    }
    return true;
}

console.log(bb(arr));