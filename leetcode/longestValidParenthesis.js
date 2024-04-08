const longestValidParentheses = function(s) {
// bb = bracket
    const bb = [];
    const bbIdx = [];

    const bbSeenIdx = []; //valid idx for opening bracket
    for(let i = 0; i < s.length; i++) {
        const b = s[i];
        if(b === '(') {
            bb.push(b);
            bbIdx.push(i)
        }else {
            if (bb.length && bb[bb.length - 1] === '(') {
                const ii = bbIdx.pop(); //pop index of the valid opening bracket
                bbSeenIdx.push(ii)
                bb.pop();
            } else {
              bb.push(b);
              bbIdx.push(i)
            }
        }
    }
    bbSeenIdx.sort((a, b) => a-b)
    let max = 0;
    if(bbSeenIdx.length) {
      max = 2;
    }
    let count = max;
    for(let i = 0; i < bbSeenIdx.length - 1; i++) {
      if(!findOverlap(bbSeenIdx[i], bbSeenIdx[i + 1], bbIdx)) {
        count += 2
      } else {
        max = Math.max(max, count);
        count = 2;
      }
    }
    max = Math.max(max, count);
    return max;
}

function findOverlap(num1, num2, bbIdx) {
  for(const idx of bbIdx) {
    if(idx > num1 && idx < num2) {
      return true
    }
  } return false
}

console.log(longestValidParentheses("(()"))
console.log(longestValidParentheses(")()())"))
console.log(longestValidParentheses(""))
console.log(longestValidParentheses("()(()"))
console.log(longestValidParentheses("()(())"))
console.log(longestValidParentheses("((()))())"))
console.log(longestValidParentheses("(()))())("))
console.log(longestValidParentheses("()()))))()()("))
console.log(longestValidParentheses(")(())))(())())"))
