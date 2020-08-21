const arr = [8, 5, 2, 9, 5, 6, 3];

//time O(n^2) && space O(1);
function sorter(arr) {
    for(let i =0; i< arr.length; i++) {
        current = 0;
        for (let j = 1; j<arr.length-i; j++) {
            if (arr[current] > arr[j]) {
                swapper(current, j);
                current = j;
                continue;
            }
            current = j;
        }
    }
    return arr;
}

function swapper(current, j) {
    [ arr[j], arr[current] ] = [ arr[current], arr[j] ];
}

console.log(sorter(arr));