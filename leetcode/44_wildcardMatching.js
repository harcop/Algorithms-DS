const isMatch = function(s, p) {
  let sIndex = 0;
  let pIndex = 0;
  let sStar = -1;
  let pStar = -1;

  while (sIndex < s.length) {
    if (pIndex < p.length && (p[pIndex] === s[sIndex] || p[pIndex] === '?')) {
        sIndex++;
        pIndex++;
    } else if (pIndex < p.length && p[pIndex] === '*') {
        sStar = sIndex;
        pStar = pIndex;
        // pIndex += 1;
        pIndex++;
    } else if (pStar !== -1) {
        pIndex = pStar + 1;
        sIndex = sStar + 1;
        sStar++;
    } else {
        return false;
    }
  }

  while (pIndex < p.length && p[pIndex] === '*') {
    pIndex++;
  }

  return pIndex === p.length;
};

// console.log(isMatch("aa", "*a"))
// console.log(isMatch("aafbc", "*"))
// console.log(isMatch("aabcde", "*bc*"))
// console.log(isMatch("cb", "?a"))
// console.log(isMatch("ab", "*a*b"))
// console.log(isMatch("ab", "*b"))
console.log(isMatch("a", "*"))
