
const combinationSum2 = function(candidates, target) {
  candidates.sort((a, b) => a-b) 
  const result = [];
  const memo = [];
  solver(candidates, target, 0, 0, [], result, memo);
  return result.map(item => item.split('-').map(ele => Number(ele)))
};

function solver(candidates, target, currentValue, start, acc, result, memo) {
  for(let i = start; i < candidates.length; i++) {
    let sum = currentValue + candidates[i];
    if(sum === target) {
      acc.push(candidates[i])
      if(!result.includes(acc.join('-'))) {
        result.push(acc.join('-'));
      }
      acc.pop();
      continue;
    }
    if(sum > target) {
      break;
    }
    acc.push(candidates[i]);
    if(!memo.includes(acc.join('-'))) {
      memo.push(acc.join('-'))
      solver(candidates, target, sum, i + 1, [...acc], result, memo);
    }
    acc.pop();
  }
}

console.log(combinationSum2([1, 1, 4], 6))
console.log(combinationSum2([10,1,2,7,6,1,5], 8))
console.log(combinationSum2([2,5,2,1,2], 5))
console.log(combinationSum2([1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1], 30))
