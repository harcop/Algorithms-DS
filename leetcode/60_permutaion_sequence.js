/**
 * @param {number} n
 * @param {number} k
 * @return {string}
 */
var getPermutation = function(n, k) {
  const tik = performance.now()
    const arr = [];
    for(let i = 1; i <= n; i++) {
      arr.push(i)
    }

    if(arr.length === 1) {
      return String(arr[0]);
    }
    const permutation = (arr, depth) => {
      if(arr.length === 1) {
        return arr;
      }
      const inner = [];
      for(let i = 0; i < arr.length; i++) {
        let ele = arr[i];
        let rem = [...arr.slice(0, i), ...arr.slice(i+1)];
        for(const per of permutation(rem, depth + 1)) {
          inner.push(String(ele) + String(per));
          if(depth === 1) {
            if(inner.length === k) {
              return inner[k-1]
            }
          }
        }
      }
      return inner;
    }

    const tok = performance.now()
    console.log(tok-tik)
    return permutation(arr, 1)
};

// console.log(getPermutation(3, 1))
console.log(getPermutation(9, 296662))

// let i = 2;
// let n = [1, 2, 3, 4]
// console.log([...n.slice(0, i), ...n.slice(i+1)])


var getPermutation2 = function (n, k) {
    let digits=""
    for (let i=1; i<=n; i++){
        digits+=String(i)
    }

    let cNFactorial=1
    for (let i=1; i<=n; i++){
        cNFactorial*=i
    }

    let currentK=k
    let ret=""
    for (let currentN=n; currentN>0 ; currentN--){
        console.log(currentK, currentN)
        let pos=Math.trunc((currentK-1)*currentN/cNFactorial)
        console.log(pos)
        ret+=digits[pos]
        currentK-=(pos*cNFactorial/currentN)
        cNFactorial/=currentN
        digits=digits.substring(0, pos)+digits.substring(pos+1)
        console.log(digits)
    }
    return ret
};

console.log(getPermutation2(3, 2))
// console.log(getPermutation2(9, 296662))
