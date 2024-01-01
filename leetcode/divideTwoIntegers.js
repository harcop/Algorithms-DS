/**
 * @param {number} dividend
 * @param {number} divisor
 * @return {number}
 */
var divide = function(dividend, divisor) {
   
    let ff = dividend < 0 || divisor < 0
    if(dividend < 0 && divisor < 0) {
        ff = !ff
    }
    dividend = Math.abs(dividend)
    divisor = Math.abs(divisor)
    let counter = 0
    if(divisor === 1) {
        counter = dividend;
    }
    else if(dividend === divisor) {
        counter = 1
    }
    else {
        while(dividend >= divisor) {
            dividend -= (divisor + divisor) // twice faster
            counter += 2;
        }
        if(dividend < 0) { // when we over remove
            counter -= 1
        }
    }
    counter = ff ? -counter : counter;
    if (counter > 2**31 - 1) {
        return 2**31 - 1
    }
    if(counter < -(2** 31)) {
        return -(2**31)
    }
    return counter
};
