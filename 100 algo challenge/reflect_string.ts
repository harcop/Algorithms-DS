function reflectString(str: string): string {
    let _s = str.toLowerCase().split("");
    let _h = 97;
    let nS = [];
    _s.forEach(ele => {
        let _e = ele.charCodeAt(0);
        _e -= _h;
        _e = 25 - _e;
        let _g = String.fromCharCode(_e+_h);
        nS.push(_g);
    });
    return nS.join("");
}

console.log(reflectString("name"));
