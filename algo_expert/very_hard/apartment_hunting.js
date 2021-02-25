const app = [
    ['sc'],
    ['g'],
    ['g', 'sc'],
    ['sc'],
    ['sc', 'st']
];
const r = ['g', 'sc', 'st'];

function aph () {
    let _f = [];
    for(let i = 0; i < app.length; i++) {
        let _t = [];
        for(let j = 0; j < r.length; j++) {
            let ele = r[j];
            let fd = app.length;
            for(let k = 0; k < app.length; k++) {
                if (app[k].includes(ele)) {
                    let sd = Math.abs(i-k);
                    let d = Math.min(fd, sd);
                    fd = d;
                }
            }
            _t.push(fd);
        }
        _f.push(Math.max(..._t));
    }
    return _f.indexOf(Math.min(..._f));
}

console.log(aph())