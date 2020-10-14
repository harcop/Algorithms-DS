const arr1 = [1,2,4,7,10,11,7,12,6,7,16,18,19];
const arr2 = [1,2,3,4,5,-1];

 //time o(n) && space O(1);
function sas (arr) {
    let _f = -1;
    let _s = -1;
    let _m = 0;
    for(let i=0; i<arr.length; i++) {
        let _df = arr[i] > arr[i+1];
        if (_df) {
            if (_f < 0) {
                console.log("sda");
                _f = i;
                _s = i;
            }
        }
        else if (arr[i] < _m) { 
            _s = i;
        }
        _m = Math.max(_m, arr[i]);
    }
    let _ss = arr.slice(_f, _s+1);
    _ss.sort((a,b) => a - b);
    for(let i=0; i<arr.length; i++) {
        if (arr[i] > _ss[0]) {
            _f = i;
            break;
        }
    }
    return [_f, _s];
}

console.log(sas(arr1));
console.log(sas(arr2));