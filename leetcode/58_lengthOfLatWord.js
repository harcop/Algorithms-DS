/**
 * @param {string} s
 * @return {number}
 */
var lengthOfLastWord = function(s) {
    let st = s.trim().split(' ');
    return st[st.length - 1].length
};

console.log(lengthOfLastWord("Hello World"))
console.log(lengthOfLastWord("   fly me   to   the moon  "))
