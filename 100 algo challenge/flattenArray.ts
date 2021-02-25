
function flattenArray(arr: any[]): any[] {
    let _l = [];
    function flat(_arr) {
        if (Array.isArray(_arr)) {
            _arr.forEach(ele => {
                flat(ele)
            })
        }
        else {
            _l.push(_arr);
        }
    }
    flat(arr);
    return _l;
}

console.log(flattenArray([[["a"]], [["b"]]]));
console.log(flattenArray([1, [2], [3, [[4]]]]));
