const arr = [8,5,2,3,5,6,9];

//tome o(log(n)) and space O(log(n))
function qs(arr, low, high) {
    const p = partition(arr, low, high);
    if (p > 0) {
        qs(arr,0,p)
        qs(arr,p,high)
    }
    return arr;
}

function partition(arr, low, high) {
    let p = 0;
    for(let i = low+1; i<high; i++) {
        if (arr[i] < arr[p]) {
            swap(i, p);
            p = i;
        } 
    }
    return p;
}

function swap(a,b) {
    [arr[a],arr[b]] = [arr[b],arr[a]];
}

console.log(qs(arr, 0, arr.length))