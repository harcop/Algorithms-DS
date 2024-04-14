const firstMissingPositive = function(nums) {
    let nn = {0:0};
    for(const element of nums) {
        if(element >= 1) {
          nn[element] = element
        }
    }
    let mm = Object.values(nn)
    for(let i = 1; i < mm.length; i++) {
        if(mm[i] !== i) {
            return i
        }
    }
    return mm[mm.length - 1] + 1 
};

console.log(firstMissingPositive([1,2,0]))
console.log(firstMissingPositive([3,4,-1,1]))
