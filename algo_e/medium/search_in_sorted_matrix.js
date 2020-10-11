const arr = [
    [1,4,7,12,15,1000],
    [2,5,19,31,32,1001],
    [3,8,24,33,35,1002],
    [40,41,42,44,45,1003],
    [99,100,103,106,128,1004]
];
const k = 99;

//time O(N+M) && space O(1);
function ss(arr, k) {
    let _t = arr[0].length-1;
    for(let i=0; i<arr.length; i++) {
        let _a = arr[i];
        for (let j=_t; j>=0; j--) {
            console.log(_a[j])
            if (_a[j] === k) {
                return [i,j];
            }
            else if (_a[j] > k) {
                _t--;
                continue;
            }
            else if (_a[j] < k) {
                break;
            }
        }
    }
    return [-1,-1];
}

console.log(ss(arr,k))