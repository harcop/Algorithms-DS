
function pd (str) {
    let _s = str.split("");
    let _f = '';
    _s.forEach(ele => {
        _f += ele.repeat(3);
    });
    return _f;
}

console.log(pd("Hello"))