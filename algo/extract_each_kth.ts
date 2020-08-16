
function extractEachKth(arr:number[], k: number): number[] {
    let i = [];
    if (k === 1) {
        arr.splice(0,1);
        return arr;
    }
    arr.forEach((ele,index) => {
        if ((index+1)%k !== 0) {
            i.push(ele);
        }
    });
    return i;
}

console.log(extractEachKth([1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 2));