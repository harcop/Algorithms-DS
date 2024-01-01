/**
 * @param {string} s
 * @return {number}
 */
var lengthOfLongestSubstring = function(s) {
    let arr = [];
    let max = 0;
    for(let i= 0; i < s.length; i++) {
        let val = s[i];
        if(arr.includes(val)) {
           let idx = arr.indexOf(val) + 1;
            arr = arr.slice(idx);
        }
        arr.push(val);
        max = Math.max(arr.length, max)
    }
    return max;
};
