const arr1 = [-1,5,10,20,28,3];
const arr2 = [26,134,135,15,17];

// time O(N^2) + MLog(M)) && space O(1)
function sdw(arr1, arr2) {
    arr1.sort((a,b) => a-b);
    arr2.sort((a,b) => a-b);

    let rp = 0;
    let pair = [];
    let _d = Infinity
    for (let i =0; i< arr1.length; i++) {
        for (let j=rp; j<arr2.length; j++) {
            let _id = Math.abs(arr1[i] - arr2[j]);
            if (_id < _d) {
                _d = _id;
                pair = [i, j];
            }
            if (arr1[i] < arr2[j]){
                break;
            }
            else if (arr1[i] > arr2[j]){
                rp++;
                continue;
            }
        }
    }
    return [arr1[pair[0]], arr2[pair[1]]];
}

// time O(NLog(N) + MLog(M)) && space O(1)
//sorting and linear iteration
function sd(arr1, arr2) {
    arr1.sort((a,b) => a-b);
    arr2.sort((a,b) => a-b);
    let lp = 0;
    let rp = 0;
    let _d = Infinity;
    let pair = [];
    while (lp < arr1.length && rp < arr2.length) {
        let _fn = arr1[lp];
        let _ln = arr2[rp];
        let _id = Math.abs(_fn - _ln);
        if (_id < _d) {
            pair = [lp, rp];
            _d = _id;
        } 
        if (_fn < _ln) {
            lp++;
        }
        else if (_fn > _ln) {
            rp++;
        }
        else if (_fn === _ln) {
            return [arr1[pair[0]], arr2[pair[1]]];
        }
    }
    return [arr1[pair[0]], arr2[pair[1]]];
}
console.log(sd(arr1, arr2));