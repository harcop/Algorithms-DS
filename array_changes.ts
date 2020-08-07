
function arrayChange(arr: number[]): number {
    let falser = true;
    let init = arr[0];
    let i = 1;
    let m = 0;
    while (falser) {

        if(i === arr.length) {
            console.log(i)
            falser = false;
        }
        if (arr[i] <= init) {
            m++;
            arr[i]++;
            if (arr[i] > init) {
                init = arr[i];
                i++;
            }
        }
        else {
            init = arr[i];
            i++;
        }
    }
    return m;
}

console.log(arrayChange([1, 1, 1]));
console.log(arrayChange([1, 4, 2]));
console.log(arrayChange([1, 2, 2]));