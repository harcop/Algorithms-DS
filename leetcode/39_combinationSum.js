const solve = (candidates, target, output, res, start, sum) => {
    if (sum > target) {
        return;
    }
    if (sum === target) {
        res.push(output);
        return;
    }
    
    for (let i = start; i < candidates.length; i++) {
        const num = candidates[i];
        output.push(num);
        sum += num;
        solve(candidates, target, [...output], res, i, sum);
        const poppedElement = output.pop();
        sum -= poppedElement;

    }
}

var combinationSum = function(candidates, target) {
    const output = [];
    const res = [];
    let sum = 0;
    solve(candidates, target, [...output], res, start = 0, sum = 0);
    return res;
};

console.log(combinationSum([2, 3, 6, 7], 7))
console.log(combinationSum([2,3,5], 8))
console.log(combinationSum([2], 1))
console.log(combinationSum([8,7,4,3], 11))
console.log(combinationSum([3, 7], 17))
console.log(combinationSum([2, 3, 7], 18))
