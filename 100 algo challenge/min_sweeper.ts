function minesweeper(m: boolean[][]): number[][] {
    console.log(m[0][1])
    let _f = [];
    for(let y = 0; y< m.length; y++) {
        let _a = [];
        for (let x = 0; x<m[y].length; x++) {
            let _ne = getN(x,y,2);
            let _c = 0;
            console.log(_ne)
            _ne.forEach(ele => {
                let _x = ele.x;
                let _y = ele.y;
                if (m[_y][_x] === true) {
                    _c++;
                }
            })
            _a.push(_c);
        }
        _f.push(_a);
    }
    return _f;
}
// i = y; j=x
function getN(x, y, d) {
    let neigh = [];
    let up = {
        x,
        y: y-1
    };
    if (y > 0) {
        neigh.push(up);
    }
    let upE = {
        x: x+1,
        y: y-1
    };
    if (x < d && y > 0) {
        neigh.push(upE);
    }
    let right = {
        x: x+1,
        y
    }
    if (x < d) {
        neigh.push(right);
    }
    let downE = {
        x: x+1,
        y: y+1
    }
    if (x < d && y < d) {
        neigh.push(downE);
    }
    let down = {
        x,
        y: y+1
    }
    if (y < d) {
        neigh.push(down);
    }
    let downW = {
        x: x-1,
        y: y+1
    }
    if (x>0 && y<d) {
        neigh.push(downW);
    }
    let left = {
        x: x-1,
        y
    }
    if (x > 0) {
        neigh.push(left);
    }
    let upW = {
        x: x-1,
        y: y-1
    }
    if (x > 0 && y > 0) {
        neigh.push(upW);
    }
    return neigh
}
console.log(minesweeper([[true, false, false],
[false, true, false],
[false, false, false]]));