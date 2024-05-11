/**
 * @param {number} x
 * @return {number}
 */
var mySqrt = function(x) {
    if(x < 2) return x
    let sqr = 2;
    while(Math.floor(sqr * sqr) < x) {
        sqr++
    }
    if(Math.floor(sqr * sqr) === x) {
        return sqr
    }
    return sqr - 1;
};

console.log(mySqrt(4))
console.log(mySqrt(8))
console.log(mySqrt(0))
