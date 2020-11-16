const arr = [];
const w = 4;
const h = 4;

for (let i = 0; i < h; i++) {
    arr.push([]);
    for (let j = 0; j< w; j++) {
        arr[i][j] = 0;
    }
}

function init () {
    const x1 = Math.floor(Math.random() * 4); 
    const y1 = Math.floor(Math.random() * 4); 
    const x2 = Math.floor(Math.random() * 4); 
    const y2 = Math.floor(Math.random() * 4); 
    console.log(x1,x2,y1,y2);
    arr[x1][y1] = randBtw(2,4)
    arr[x2][y2] = randBtw(2,4)
}

function randBtw (...num) {
    const _f = {};
    const _n = num.length * 3;
    let max = 0;
    let p = -1;
    for (let i = 0; i < _n; i++) {
        const pos = Math.floor(Math.random() * num.length);
        if (_f[pos] === undefined) {
            _f[pos] = 0;
        }
        _f[pos] += 1;
        if (_f[pos] > max) {
            max = _f[pos];
            p = pos;
        }
    }
    return num[p];
}
init();
console.log(arr);