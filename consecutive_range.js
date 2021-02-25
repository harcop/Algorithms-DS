const arr = [-2,4,5,6,7,8,10,11,15,16,17,18,21];
const _newArray = [];
let start = arr[0];
let end = start;
for(let i=1; i<=arr.length; i++) {
    let elem = arr[i];
    if (elem === end+1) {
        end = elem; // update the end value (range)
    }else {
        if (end !== start) {
            _newArray.push(`${start}-${end}`);
        } else {
            _newArray.push(start);
        }
        start = elem;
        end = start;
    }
}

console.log(_newArray.join(','))