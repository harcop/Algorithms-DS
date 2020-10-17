const arr = [3,4,2,1,2,3,7,1,1,1,3];

//time O(n^2) && space O(n);
function mj1() {
    let _t = Array(arr.length);
    _t.fill(0,0,arr.length);
    // console.log(_t);
    for(let i=1; i<arr.length; i++) {
        for (let j=0;j<i;j++) {
            if(arr[j]+j >= i) {
                _t[i] = _t[j]+1;
                break;
            }
        }
    }
    return _t[_t.length-1]
}

//time O(n) && space O(1);
function mj () {
    let maxReach = arr[0];
    let steps = arr[0];
    let jumps = 0;
    for (let i=1; i<arr.length; i++) {
        maxReach = Math.max(maxReach, arr[i]+i) 
        steps -= 1;
        if (steps === 0) {
            jumps += 1;
            steps = maxReach - i;
        } 
    }
    return jumps + 1;
}

console.log(mj());