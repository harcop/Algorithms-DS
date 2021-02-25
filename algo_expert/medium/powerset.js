const arr = [1,2,3];
// [[],[1],[2],[1,2],[3],[1,3],[2,3],[1,2,3]]
function ps (arr) {
    let pp = [[]];
    arr.forEach(ele => {
        let _pp = pp.length;
        for(let i=0; i<_pp; i++) {
            let _np = [...pp[i]];
            _np.push(ele);
            console.log(_np);
            pp.push(_np);
        }

    });
    return pp;
}

console.log(ps(arr));