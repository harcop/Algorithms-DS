const arr = [];
const w = 9;
const pos = [];
const _g = [1,2,3,4,5,6,7,8,9];

for (let y = 0; y < w; y++) {
    arr.push([]);
    for (let x = 0; x < w; x++) {
        arr[y][x] = 'z';
        // $('#gridBox').append(`<div class="smallBox" id="${y}${x}">0</div>`);
    }
    // $('#gridBox').append('<br/>');
    pos.push([..._g]);
}
console.log(pos)
// console.log(arr)

for(let i = 1; i < 10; i++) {
    // const _pos = [1,2,3,4,5,6,7,8,9];
    const _v = [];
    for(let j = 0; j < arr.length; j++) {
        // console.log(i, j, pos[5])
        const _x = pos[j];
        const _visitedBefore = removeVisited(_v, _x);
        let _r = randBtw(..._x);
        _v.push(_r);
        console.log(_r, _x);
        arr[j][_r-1] = i;
        _x.splice(_x.indexOf(_r), 1);
        _x.push(..._visitedBefore);
        // console.log(pos);
    }
    console.log(arr);
    console.log(_v)
}

function removeVisited(_v, _x) {
    let _f = [];
    for (let v of _v) {
        if(_x.includes(v)) {
            _f.push(v);
            _x.splice(_x.indexOf(v), 1);
        }
    }
    return _f;
}
console.log(arr);



// function init () {
//     const x1 = Math.floor(Math.random() * 4); 
//     const y1 = Math.floor(Math.random() * 4); 
//     const x2 = Math.floor(Math.random() * 4); 
//     const y2 = Math.floor(Math.random() * 4); 
//     console.log(x1,x2,y1,y2);
//     arr[y1][x1] = randBtw(2,4)
//     arr[y2][x2] = randBtw(2,4)
//     $(`#${y1}${x1}`).text(arr[y1][x1])
//     $(`#${y2}${x2}`).text(arr[y2][x2])
// }

//[1,2,3,4,5,6,7,8,9]
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



// init();
// swapDown();
// console.log(arr);