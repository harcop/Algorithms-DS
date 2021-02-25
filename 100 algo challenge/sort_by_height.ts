function sortByHeight(a: number[]): number[] {
    let _t = [];
    a.forEach((ele, index) => {
        if (ele === -1) {
            _t.push(index);
        }
    });
    a.sort((a, b) => a-b);
    a.splice(0, _t.length);
    _t.forEach(ele => {
        a.splice(ele,0,-1)
    })
    return a;
}

console.log(sortByHeight([-1, 150, 190, 170, -1, -1, 160, 180]));