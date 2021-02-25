const arr = [2, 3, 1, -4, -4, 2];
const arr2 = [1,1,-2];

function scc (arr) {
    let _it = [0];
    let falser = true;
    let i = 0;
    let _p = 0;
    while(falser) {
        i += arr[i];
       if (i > arr.length-1) {
           i = i - arr.length;
       }
       else if (i < 0) {
            i = arr.length + i;
       }
       _p = arr[i];
       if (_it.includes(i)) {
           console.log(_it)
            if (_it.length === arr.length && i === 0) {
                return true;
            }
            return false;
       }
       _it.push(i)
    }
}

console.log(scc(arr));
console.log(scc(arr2));