const arr = [8,5,2,9,5,6,3];

//time o(log(n)) and space O(log(n))
function qs(arr, low, high) {
    const p = partition(arr, low, high);
    if (p > low) {
        qs(arr,0,p)
        console.log(arr)
        qs(arr,p+1,high)
    }
    return arr;
}

function partition(arr, low, high) {
    let p = low;
    console.log(low, high);
    console.log(arr.slice(low, high))
    for(let i = low+1; i<high; i++) {
        if (arr[i] < arr[p]) {
            swap(i, p);
            console.log(p)
            p = i;
        } else {
            p = i;
        }
    }
    console.log(p)
    return p;
}

function swap(a,b) {
    [arr[a],arr[b]] = [arr[b],arr[a]];
}

console.log(qs(arr, 0, arr.length))