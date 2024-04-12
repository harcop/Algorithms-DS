/**
 * @param {number} n
 * @return {string}
 */
var countAndSay = function(n) {
    return cns(n)
};

function cns(n) {
    if(n === 1) {
        return 1
    }
    let nn = 1;
    let currentSay = 1;
    while(nn < n) {
        currentSay = say(currentSay);
        nn += 1;
    }
    return currentSay
}

function say(val) {
  let vv = String(val)
    const s = [];
    let currentV = vv[0];
    let currentCount = 0;
    for(let i = 0; i < vv.length; i++) {
        let v = vv[i];
        if(currentV === v) {
            currentCount += 1;
            if(vv[i + 1] !== v) {
              s.push(`${currentCount}${currentV}`);
              currentV = vv[i + 1];
              currentCount = 0;
            }
        }
    }
    return s.join('')
}


console.log(countAndSay(1))
console.log(countAndSay(2))
console.log(countAndSay(3))
console.log(countAndSay(4))
console.log(countAndSay(5))
