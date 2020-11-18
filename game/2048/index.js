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

function newDigit () {
    let _t = []
    arr.forEach((row, y) => {
        row.forEach((col, x) => {
            if (col === 0) {
                _t.push(`${y}${x}`);
            }
        })
    })
    return _t;
}

function placeNewDigit () {
    const _t = newDigit();
    const pos = randBtw(..._t)
    const _p = pos.split('');
    const [y, x] = _p;
    arr[y][x] = randBtw(2,4);
}

let _m = false;
function swapUp () {
    for(let x = 0; x < w; x++) {
        _m = false;
        for (let y = 1; y < h; y++) {
            let ele = arr[y][x];
            if (ele === 0) {
                continue;
            } else {
                loopUp(ele, x, y);
            }    
        }
    }
}
function swapDown () {
    for(let x = 0; x < w; x++) {
        _m = false;
        for (let y = h-2; y >= 0; y--) {
            let ele = arr[y][x];
            if (ele === 0) {
                continue;
            } else {
                loopDown(ele, x, y);
            }    
        }
    }
}

function swapLeft () {
    for(let y = 0; y < h; y++) {
        _m = false;
        for (let x = 1; x < w; x++) {
            let ele = arr[y][x];
            if (ele === 0) {
                continue;
            } else {
                loopLeft(ele, x, y);
            }    
        }
    }
}

function swapRight () {
    for(let y = 0; y < h; y++) {
        _m = false;
        for (let x = w - 1; x >= 0; x--) {
            let ele = arr[y][x];
            if (ele === 0) {
                continue;
            } else {
                loopRight(ele, x, y);
            }    
        }
    }
}

function arrr() {
    let _f = '';
    arr.forEach(ele => {
        _f += `${ele}\n`;
    })
    return _f;
}

function loopUp (ele, x, y) {
    if (y > 0 && ele > 0) {
        let _y = y - 1;
        let up = arr[_y][x];
        if (up === 0) {
            console.log(up)
            arr[y][x] = 0;
            arr[_y][x] = ele;
            loopUp(ele, x, _y);
        } else {
            console.log(ele, up)
            if(ele === up && !_m) {
                console.log('am')
                arr[y][x] = 0;
                arr[_y][x] = ele * 2;
                _m  = true;
            }
        }
    }
}

function loopDown (ele, x, y) {
    if (y < h - 1 && ele > 0) {
        let _y = y + 1;
        let down = arr[_y][x];
        if (down === 0) {
            console.log(down)
            arr[y][x] = 0;
            arr[_y][x] = ele;
            loopDown(ele, x, _y);
        } else {
            console.log(ele, down)
            if(ele === down && !_m) {
                console.log('am')
                arr[y][x] = 0;
                arr[_y][x] = ele * 2;
                _m  = true;
            }
        }
    }
}

function loopRight (ele, x, y) {
    if (x < w - 1 && ele > 0) {
        let _x = x + 1;
        let right = arr[y][_x];
        if (right === 0) {
            console.log(right)
            arr[y][x] = 0;
            arr[y][_x] = ele;
            loopRight(ele, _x, y);
        } else {
            console.log(ele, right)
            if(ele === right && !_m) {
                console.log('am')
                arr[y][x] = 0;
                arr[y][_x] = ele * 2;
                _m  = true;
            }
        }
    }
}

function loopLeft (ele, x, y) {
    if (x > 0 && ele > 0) {
        let _x = x - 1;
        let left = arr[y][_x];
        if (left === 0) {
            console.log(left)
            arr[y][x] = 0;
            arr[y][_x] = ele;
            loopLeft(ele, _x, y);
        } else {
            console.log(ele, left)
            if(ele === left && !_m) {
                console.log('am')
                arr[y][x] = 0;
                arr[y][_x] = ele * 2;
                _m  = true;
            }
        }
    }
}
init();
placeNewDigit();
console.log(arrr());
// swapUp();
swapRight()
swapDown()
console.log(arrr());
// swapDown();
// console.log(arr);