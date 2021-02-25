const arr = [8,5,2,5,9,6,9,3];

// time O9n^2) && space O(1)
function is(arr) {
    for (let i=1; i<arr.length;i++) {
        let point = i;
        while(point > 0 && arr[point] < arr[point-1]) {
            swapper(point);
            point--;
        }
    }
    return arr;
}

function swapper(point) {
    [arr[point], arr[point-1]] = [arr[point-1], arr[point]];
}

console.log(is(arr));