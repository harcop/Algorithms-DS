const points = [
    {
        x : 5, y : 10
    },
    {
        x : 20, y : 0
    },
    {
        x : 6, y : 20
    },
    {
        x : 21, y : 10
    },
];

function fs () {
    let xd = -1;
    let yd = -1;
    for (let i = 0; i < points.length; i+=2) {
        console.log(points[i])
        if (xd === -1) {
            let x1 = points[i].x;
            let x2 = points[i+1].x;
            xd = Math.abs(x2-x1);
        } else {
            let _x1 = points[i].x;
            let _x2 = points[i+1].x;
            let _d = Math.abs(_x2-_x1);
            if (_d !== xd) {
                return false;
            }
        }
        if (yd === -1) {
            let y1 = points[i].y;
            let y2 = points[i+1].y;
            yd = Math.abs(y2-y1);
        } else {
            let _y1 = points[i].y;
            let _y2 = points[i+1].y;
            let _d = Math.abs(_y2-_y1);
            if (_d !== yd) {
                return false;
            }
        }
    }
    return true;
}

console.log(fs());