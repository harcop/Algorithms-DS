const arr1 = [
    [1,3,4,10],
    [2,5,9,11],
    [6,8,12,15],
    [7,13,14,16]
]
const arr2 = [
    [1,3,4,10,11],
    [2,5,9,12,17],
    [6,8,13,16,18],
    [7,14,15,19,20]
]

function zz (arr) {
    let h = arr.length-1;
    let w = arr[0].length-1;
    let _d = "d";
    let _x = 0;
    let _y = 0;
    let _f = [];
    let falser = true;
    let _p = [_y,_x];
    _f.push(arr[_p[0]][_p[1]])
    let _sw = false;
    while (falser) {
        if (_d === "d") {
            _sw = false
            console.log('am here d');
            if (_y+1 <= h && _x === 0) {
                _sw = true;
                _d = "ur"
            }
            else if (_y+1 < h && _x === w || _y+1 === h) {
                _sw = true;
                _d = "dl"
            }
            if (_sw) {
                _y += 1;
                _p = [_y,_x];
                _f.push(arr[_p[0]][_p[1]])
            } else {
                break;
            }
        }
        if (_d === "ur") {
            _sw = false
            console.log('am here ur');
            if (_y-1 === 0 && _x+1 < w) {
                _sw = true;
                _d = "r"
            }
            else if ((_y-1 === 0 && _x+1 <= w) || (_y-1 > 0 && _x+1 === w)) {
                _sw = true;
                _d = "d"
            }
            else if (_y-1 > 0 && _x+1 < w) {
                _sw = true;
                _d = "ur"
            }
            if (_sw) {
                _x += 1;
                _y -= 1;
                _p = [_y,_x];
                _f.push(arr[_p[0]][_p[1]])
            } else {
                break;
            }
        }
        if (_d === "r") {
            _sw = false
            console.log('am here r');
            if (_y === 0) {
                _sw = true;
                _d = "dl"
            }
            else if (_y === h) {
                _sw = true;
                _d = "ur"
            }
            if (_sw) {
                _x += 1;
                _p = [_y,_x];
                _f.push(arr[_p[0]][_p[1]])
            } else {
                break;
            }
        }
        if (_d === "dl") {
            _sw = false
            console.log('am here dl');
            if (_y+1 < h && _x-1 > 0) {
                _sw = true;
                _d = "dl"
            }
            else if (_x-1 === 0 && _y+1 < h) {
                _sw = true;
                _d = "d"
            }
            else if (_y+1 === h && _x > 0) {
                _sw = true;
                _d = "r"
            }
            if (_sw) {
                _x -= 1;
                _y += 1;
                _p = [_y,_x];
                _f.push(arr[_p[0]][_p[1]])
            } else {
                break;
            }
        }
        else if (_f.length === arr.length) {
            break;
        }
    }
    return _f;
}

console.log(zz(arr1));
console.log(zz(arr2));