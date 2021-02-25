function checkPalindrome(str: string): boolean {
    let opp = str.split("").reverse().join("");
    if (str.toLowerCase() === opp.toLowerCase()){
        return true;
    }
    return false;
}

console.log(checkPalindrome('aabaa'));
console.log(checkPalindrome('abac'));
console.log(checkPalindrome('a'));
