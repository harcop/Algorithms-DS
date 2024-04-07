/**
 * @param {string} str
 * @return {number}
 */
const lengthOfLongestSubstring = function(str) {
    let arr = [];
    let max = 0;
    for(const s of str) {
        let val = s;
        if(arr.includes(val)) {
           let idx = arr.indexOf(val) + 1;
            arr = arr.slice(idx);
        }
        arr.push(val);
        max = Math.max(arr.length, max)
    }
    return max;
};
