const multiply = function(num1, num2) {
  if(num1 === "0" || num2 === "0") {
    return '0';
  }
  let total = new Array(num1.length + num2.length).fill(0);
  for(let i = num1.length - 1; i >= 0; i--) {
      let currentI = num1[i]
      for(let j = num2.length - 1; j >= 0; j--) {
        let currentJ = num2[j]
        let p1 = i + j;
        let p2 = p1 + 1;
        let sum = total[p2] + Number(currentI) * Number(currentJ);
        total[p2] = sum%10;
        total[p1] += Math.floor(sum/10);
      }
  }
  if(total[0] === 0) {
    total.shift()
  }
  return String(total.join(''));
};

console.log(multiply("2", "3"))
console.log(multiply("123", "456"))
console.log(multiply("89", "21"))
console.log(multiply("123456789", "987654321"))
