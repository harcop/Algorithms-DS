const arr = [5,2,[7,-1],3,[6,[-13,8],4]];
let s = 0;
function ps(arr, d=1) {
    let _s = 0;
    arr.forEach((ele, index) => {
        if (Array.isArray(ele)) {
            const r = ps(ele, d+1);
            arr[index] = r;
            _s += r;
        }
        else {
            _s += ele;
        }
    })
    return _s*d;
}

console.log(ps(arr));