const permute = function(nums) {
    const pp = findPermutation(nums);
    return pp.map(ele => ele.split(',').map(num => Number(num)))
};

function findPermutation(nums) {
  const per = []
    if(nums.length === 1) {
        return [nums.join('')]
    }
    for(let i = 0; i < nums.length; i++) {
        const currentEle = nums[i]
        const rem = [...nums.slice(0, i), ...nums.slice(i+1)]
        for(const permutation of findPermutation(rem)) {
            const bb = `${currentEle}, ${permutation}`;
            if(!per.includes(bb)) {
              per.push(bb)
            }
        }
    }
    return per
}

// console.log(permute('ab'))
console.log(permute([1, 1, 2]))
console.log(permute([0, 1]))
console.log(permute([1]))
