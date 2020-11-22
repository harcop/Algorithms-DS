const c1 = ['09:00-10:30', '12:00-13:00', '16:00-18:00'];
const c1D = ['09:00-20:00'];
const c2 = ['10:00-11:30', '12:30-14:30', '14:30-15:00', '16:00-17:00'];
const c2D = ['10:00-18:30'];
const k = '30';

//time O(c1+c2) && space O(c1+c2)

function sc(c) {
    let _f = [];
    c.forEach(ele => {
        let _n = ele.split('-');
        let a = _n[0].split(':').join('');
        let b = _n[1].split(':').join('');
        _f.push([a,b])
    })
    return _f;
}

function ic(c, cd) {
    let _i = c[0][0];
    let _l = cd[0][1];
    console.log(_i, _l)
    c.unshift(['0000', _i]);
    c.push([_l, '2359']);
}
let _c1d = sc(c1D);
let _c2d = sc(c2D);
let _c1 = sc(c1);
let _c2 = sc(c2);
ic(_c1, _c1d);
ic(_c2, _c2d);

console.log(_c1);
console.log(_c2);

function merge1() {
    let falser = true;
    let _f = [];
    while (falser) {
        if (_c1.length === 0 || _c2.length === 0) {
            break;
        }
        if (_c1[0][1] <= _c2[0][1]) {
            _f.push(_c1[0]);
            _c1.splice(0,1);
        } else {
            _f.push(_c2[0]);
            _c2.splice(0,1);
        }
    }
    console.log(_c1, _c2)
    if (!_c1.length) {
        _f.push(..._c2)
    }else {
        _f.push(..._c1)
    }
    return _f;
} 
let _m = merge1();
console.log(_m);


function compress () {
    let _f = []
    let _min = _m[0][0];
    let _max = _m[0][1];
    let _o = false;
    _m.forEach(ele => {
        if (ele[0] <= _max) {
            _max = ele[1];
            if (ele[0] < _min) {
                _min = ele[0];
            }
            _o = true;
        } else {
            _f.push([_min, _max]);
            _min = ele[0];
            _max = ele[1];
            _o = false;
        }
    })
    if (_o) {
        _f.push([_min, _max]);
    }
    return _f;
}
let _t = compress()
console.log(_t);

function findTime() {
    let _f = [];
    let _dl = _t[0][1];
    _t.forEach(ele => {
       let diff = ele[0] - _dl;
       if (diff >= k) {
           _f.push([_dl, ele[0]]);
           _dl = ele[1];
       }
    })
    return _f;
}

console.log(findTime())