const arr = [7,6,4,-1,1,2];
const k = 16;
function fn () {
    let ht = {}; 
    let _f = [];
    arr.forEach((ele, index) => {
        if (Object.keys(ht).length > 0) {
            for (let i=index+1; i < arr.length; i++) {
                let p = ele + arr[i];
                let q = k - p;
                if (ht[q]) {
                    let _s = ht[q];
                    _f.push([..._s, ele, arr[i]]);
                }
            }
        }
        for (let i=0; i<index; i++) {
            let p = ele + arr[i];
            ht[p] = [ele, arr[i]];
        }
    })
    return _f;
}

console.log(fn());