const str = "this is a big string";
const arr = ['this', 'yo', 'is', 'a', 'bigger', 'string', 'kappa'];

function ms () {
    let _ht = {};
    let _s = str.split(' ');
    let _f = [];
    _s.forEach(ele => {
        _ht[ele] = true;
    });
    arr.forEach(ele => {
         if (_ht[ele]){
             _f.push('T' )
         }else{
             _f.push('F');
         }
    })
    return _f;
}

console.log(ms())