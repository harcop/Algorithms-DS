const arr = [12,3,1,2,-6,5,-8,6];
const cs = 0;

// time O(n^2) && space O(n);
function ts(arr, cs) {
    arr.sort((a,b) => a-b);
    let _f = [];
    arr.forEach((ele, index) => {
        let lp = index + 1;
        let rp = arr.length - 1;
        while (lp<rp) {
            let _s = ele + arr[lp] + arr[rp];
            if (_s === cs) {
                _f.push([ele,arr[lp], arr[rp]]);
                lp++;
                rp--;
            }
            if (_s > cs) {
                rp--;
            }
            else if (_s < cs) {
                lp++;
            }
        }
    })
    return _f;
};

console.log(ts(arr, 0));
