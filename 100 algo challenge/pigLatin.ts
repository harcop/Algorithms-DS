function pigLatin(str: string): string {
    const _v = ["a","e","i","o","u"];
    let _s = str.split("");
    let _f = [];
    let i = 0;
    while(!_v.includes(_s[i])) {
        _f.push(_s[i]);
        _s.splice(0,1);
    }
    console.log(_f);
    return _f.length > 0 ? `${_s.join("")}${_f.join("")}ay`: `${_s.join("")}way`
}

console.log(pigLatin("glove"));
console.log(pigLatin("eight"));
