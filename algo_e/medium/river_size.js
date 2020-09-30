const r = [ [1,0,0,1,0], 
            [1,0,1,0,0], 
            [0,0,1,0,1], 
            [1,0,1,0,1], 
            [1,0,1,1,0]
        ];
let vs = [];
 function rs(r) {
     let tn = [];
    for (let y = 0; y< r.length; y++) {
        for(let x = 0; x<r[y].length; x++) {
            let ps = `${x}-${y}`;
            if (r[y][x] === 1 && !vs.includes(ps)){
                tn.push(ne({x,y}, r, r[y], 0));
            }
        }
    }
    return tn;
 }

 function ne(p, r, h, tn) {
     let { x, y } = p;
     let neigh = []
     tn++;
    if (x < h.length - 1 && r[y][x+1] === 1) {
        let ps = `${x+1}-${y}`;
        if (!vs.includes(ps)) {
            vs.push(ps);
            let rf = {x: x+1,y}
            neigh.push(rf);
        }
    }
    if (x > 0 && r[y][x-1] === 1) {
        let ps = `${x-1}-${y}`;
        if (!vs.includes(ps)) {
            vs.push(ps);
            let lf = {x: x-1,y}
            neigh.push(lf);
        }
    }
    if (y < r.length-1 && r[y+1][x] === 1) {
        let ps = `${x}-${y+1}`;
        if (!vs.includes(ps)) {
            vs.push(ps);
            let dw = {x, y: y+1}
            neigh.push(dw);
        }
    }
    for (let _n of neigh) {
        tn = ne(_n, r, h, tn)
    }
    return tn;
 }

 console.log(rs(r).sort());