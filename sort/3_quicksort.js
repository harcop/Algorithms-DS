// 54386791
const arr = [5,4,3,8,6,7,9,1];

function quicksort () {
    let l = 0;
    let r = arr.length-2;
    // packing the number greater than p on the right and number greater less than p on the left; recursively sorting them;
    sort(l, r);
    return arr;
}

function sort(l, r) {
    let low = l;
    let high = r;
    let p = r+1;
    let _p = arr[p];
    if (r > l) {
        while (l <= r) {
            let _l = arr[l];
            let _r = arr[r];

            if (_l > _r) {
                swap(l, r);
            }

            if (_l < _p) {
                l += 1;
            }
            if (_r > _p) {
                r -= 1;
            }
        }
        swap(l, p);
        sort(low, l-1);
        sort(r+1, high);
    }
}

function swap(a, b) {
    [arr[a], arr[b]] = [arr[b], arr[a]]; 
}

console.log(quicksort())