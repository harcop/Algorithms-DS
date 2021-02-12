// 54386791
const arr = [5,4,3,8,6,7,9,1];
function AscBubble () {
    let swapper = true;
    let i = 0;
    while(swapper) {
        let current = arr[i];
        let temp = i;
        swapper = false;
        for (let j = i + 1; j < arr.length; j++) {
            const a = arr[j];
            if (current > a) {
                swapper = true;
                swap(temp, j);
            } else {
                current = a;
            }
            temp = j;
        }
    }
    return arr;
}

function DscBubble () {
    let swapper = true;
    let i = 0;
    while(swapper) {
        let current = arr[i];
        let temp = i;
        swapper = false;
        for (let j = i + 1; j < arr.length; j++) {
            const a = arr[j];
            if (current < a) {
                swapper = true;
                swap(temp, j);
            } else {
                current = a;
            }
            temp = j;
        }
    }
    return arr;
}

function swap(a, b) {
    [arr[a], arr[b]] = [arr[b], arr[a]]; 
}
console.log(AscBubble())
console.log(DscBubble())
