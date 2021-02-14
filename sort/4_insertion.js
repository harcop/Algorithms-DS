const arr = [5,4,3,8,6,7,9,1];

function insertion() {
    //for every element loop back to find where it belongs to. Ascending
    for(let i = 1; i < arr.length; i++) {
        let j = i; //temp
        while(arr[j] < arr[j-1]) {
            swap(j, j-1);
            j -= 1;
        }
    }
    return arr;
}


function swap(a, b) {
    [arr[a], arr[b]] = [arr[b], arr[a]]; 
}

console.log(insertion());