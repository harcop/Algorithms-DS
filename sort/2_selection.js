// 54386791
const arr = [5,4,3,8,6,7,9,1];

function selection () {
    let i = 0;
    //find the smallest among the elements and sort
    while(i < arr.length) {
        let min = arr[i];
        let _m = i;
        for (let j = i + 1; j < arr.length; j++) {
            if (arr[j] < min) {
                min = arr[j];
                _m = j
            }
        }
        swap(i, _m);
        i++;
    }
    return arr;
}

function swap(a, b) {
    [arr[a], arr[b]] = [arr[b], arr[a]]; 
}

console.log(selection());