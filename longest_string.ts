function allLongestStrings(ina: string[]): string[] {
    var maxLen = 0;
    var aIna = [];
    ina.forEach(ele => {
        if (ele.length > maxLen) {
            maxLen = ele.length;
            console.log(maxLen)
        }
    })
    ina.forEach(ele => {
        if(ele.length === maxLen) {
            aIna.push(ele);
        }
    })
    return aIna;
}

console.log(allLongestStrings(["aba", "aa", "ad", "vcd", "aba"]));