function chunkyMonkey(arr: any[], size: number): any[][] {
    let _f = [];
    while (arr.length > 0) {
        let _arr = arr.slice(0,size);
        arr.splice(0,size);
        _f.push(_arr);
    }
    return _f;
}

console.log(chunkyMonkey(["a", "b", "c", "d"], 2));
console.log(chunkyMonkey([0, 1, 2, 3, 4, 5], 4));