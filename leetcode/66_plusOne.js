/**
 * @param {number[]} digits
 * @return {number[]}
 */
var plusOne = function(digits) {
    digits.unshift(0)
    let i = digits.length - 1;
    let rem = 1;
    while(i >= 0) {
        let jj = digits[i] + rem;
        rem = Math.floor(jj / 10)
        if(rem > 0) {
          digits[i] = jj % 10
        } else { 
            digits[i] = jj
            break
        }
        i--
    }
    if(digits[0] === 0) {
        digits.shift()
    }
    return digits
};

console.log(plusOne([1,2,3]))
console.log(plusOne([4,3,2,1]))
console.log(plusOne([9]))
console.log(plusOne([6,1,4,5,3,9,0,1,9,5,1,8,6,7,0,5,5,4,3]))
