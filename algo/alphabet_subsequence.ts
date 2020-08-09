function alphabetSubsequence(s: string): boolean {
    let seq = 0;
    for(let i = 0; i < s.length; i++) {
        if (s.charCodeAt(i) <= seq) {
            return false
        }
        seq = s.charCodeAt(i);
    }
    return true;
}

console.log(alphabetSubsequence('zab'))
console.log(alphabetSubsequence('effg'))
console.log(alphabetSubsequence('cdce'))
console.log(alphabetSubsequence('ace'))
console.log(alphabetSubsequence('bxz'))