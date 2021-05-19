
function findone(arr) {
    const seen = [];
    for(let i = 0; i<arr.length; i++) {
        const ele = arr[i];
        const na = arr.slice(i+1);
        if(!na.includes(ele) && !seen.includes(ele)) {
            return ele;
        }
        seen.push(ele)
    }
}
console.log(findone([2,2,1]))
console.log(findone([12,3,3,1,12]))