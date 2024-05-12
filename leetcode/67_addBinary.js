/**
 * @param {string} a
 * @param {string} b
 * @return {string}
 */
var addBinary = function(a, b) {
    let bin = []
    a = a.split('')
    b = b.split('')
    let p1 = a.length - 1;
    let p2 = b.length - 1;
    let rem = 0;
    while(p1 >= 0 && p2 >= 0) {
      let a1 = a[p1];
      let b1 = b[p2];
      let sum = rem + Number(a1) + Number(b1);
      rem = reminder(sum, bin)
      p1 -= 1;
      p2 -= 1;
    }
    while(p1 >= 0) {
      let a1 = a[p1];
      let sum = rem + Number(a1)
      rem = reminder(sum, bin)
      p1 -= 1;
    }

    while(p2 >= 0) {
      let b1 = b[p2];
      let sum = rem + Number(b1)
      rem = reminder(sum, bin)
      p2 -= 1;
    }
    rem > 0 ? bin.push(rem) : ''
    return bin.reverse().join('')
};

function reminder(sum, bin) {
  let rem = 0
  if(sum === 2) {
    bin.push(0);
    rem = 1;
  } else if(sum === 3) {
    bin.push(1)
    rem = 1;
  } else if(sum === 1) {
    bin.push(1);
    rem = 0;
  } else {
    bin.push(0);
    rem = 0;
  }
  return rem
}

console.log(addBinary("11", "1"))
console.log(addBinary("1010", "1011"))
console.log(addBinary("0", "0"))
console.log(addBinary("1", "111"))
console.log(addBinary("1111", "1111"))
