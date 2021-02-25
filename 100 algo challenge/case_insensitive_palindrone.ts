function isCaseInsensitivePalindrome(str: string): boolean {
    let opp = str.split("").reverse().join("");
    if (str.toLowerCase() === opp.toLowerCase()){
        return true;
    }
    return false;
}

console.log(isCaseInsensitivePalindrome('AaBaa'));
console.log(isCaseInsensitivePalindrome('abac'));