const arr1 = [8,12,2,3,15,5,7];
const arr2 = [8,12,2,3,7,4,5,6,11];

//time O(n^2) && space O(n);
function ms(arr) {
    let ht = {};
    for(let ele of arr) {
        let _m = 0;
        for(let k in ht) {
            if (ele > k) {
                if (ht[k] > _m) {
                    _m = ht[k];
                }
            }
        }
        ht[ele] = ele + _m;
    }
    return Math.max(...Object.values(ht));
}

console.log(ms(arr1));
console.log(ms(arr2));