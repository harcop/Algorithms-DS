const digit = {
    1: 2,
    2: 5,
    3: 5,
    4: 4,
    5: 5,
    6: 6,
    7: 3,
    8: 7,
    9: 6,
    0: 6
};


const _ds = [2,3];

const n = 3;
let _fd = 0;
n.toString().split("").forEach(ele => {
    _fd += digit[parseInt(ele)];
});
console.log(_fd);
const _f = [];
function ls (){
    if (_fd % 2) {
        let _n =  Math.floor(_fd/2) - 1;
        let l = 7;
        while (_n > 0) {
            console.log(_n);
            l = (l * 10) + 1;
            _n -= 1;
        }
        return l;
    } 
    let l = 0;
    while (_fd > 1) {
        l = (l * 10) + 1;
        _fd -= 2;
    }
    return l;
}
console.log(ls())