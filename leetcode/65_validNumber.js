/**
 * @param {string} s
 * @return {boolean}
 */
var isNumber = function(s) {
    // no regex

    let seenE = false;
    let lastE = false;
    let seenDot = false;
    let seenDigit = false;

    let digits = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"]

    if(['e', 'E'].includes(s[0])) {
        return false;
    }
    if(s[0] === '.' && !digits.includes(s[1])) {
        return false;
    }
    if(['+', '-'].includes(s[0]) && ['e', 'E'].includes(s[1])) {
        return false;
    }

    for(let i = 0; i < s.length; i++) {
        if(digits.includes(s[i]) || ['e', 'E', '.', '-', '+'].includes(s[i])) {
            if(digits.includes(s[i])) {
              seenDigit = true
            }
            if(['e', 'E'].includes(s[i])) {
                if(seenE || ['+', '-'].includes(s[i-1])) {
                    return false
                }
                seenE = true
                lastE = true
                continue
            }
            if(['-', '+'].includes(s[i])) {
              if((s[i-1] && ['-', '+', '.'].includes(s[i-1]))) {
                return false;
              }
              if(s[i-1] && ['e', 'E'].includes(s[i-1]) && digits.includes(s[i+1])) {
                  continue
              }
              if(seenDigit) {
                return false
              }
            }
            if(s[i] === '.') {
              if(seenE || seenDot) {
                return false
              }
              if(s[i-1] && ['-', '+'].includes(s[i-1]) && s[i+1] && !digits.includes(s[i+1])) {
                return false
              }
              seenDot = true
            }
            lastE = false
        }else {
            return false
        }
    }

    return !lastE && seenDigit
};

console.log(isNumber("2"))
console.log(isNumber("0089"))
console.log(isNumber("-0.1"))
console.log(isNumber("+3.14")) 
console.log(isNumber("4."))
console.log(isNumber("-.9"))
console.log(isNumber("2e10"))
console.log(isNumber("-90E3"))
console.log(isNumber("3e+7"))
console.log(isNumber("+6e-1"))
console.log(isNumber("53.5e93"))
console.log(isNumber("-123.456e789"))
console.log(isNumber("46.e3"))
console.log(isNumber(".1"))
console.log(isNumber("+.8"))

// false
// "abc", "1a", "1e", "e3", "99e2.5", "--6", "-+3", "95a54e53".
console.log(isNumber("abc"))
console.log(isNumber("1a"))
console.log(isNumber("1e"))
console.log(isNumber("e3"))
console.log(isNumber("99e2.5"))
console.log(isNumber("--6"))
console.log(isNumber("-+3"))
console.log(isNumber("95a54e53"))
console.log(isNumber("."))
console.log(isNumber(".."))
console.log(isNumber("..9"))
console.log(isNumber(".e1"))
console.log(isNumber("6+1"))
console.log(isNumber("6+."))
console.log(isNumber(".-4"))
console.log(isNumber("+E3"))
console.log(isNumber(".+E3"))
console.log(isNumber("+.E3"))
