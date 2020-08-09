function arrayConversion(arr: number[]): number {
    let _arr = arr;
    let new_arr = [];
    let y = 1;
    while (_arr.length !== 1) {
        if (y%2 === 1) {
            console.log(y);
            for (let i = 0; i < _arr.length; i=i+2) {
                new_arr.push(_arr[i]+_arr[i+1]);
            }
        }
        else{
            console.log(y);
            for (let i = 0; i < _arr.length; i=i+2) {
                new_arr.push(_arr[i]*_arr[i+1]);
            }
        }
        _arr = new_arr;
        new_arr = [];
        y++;
    }
    return _arr[0];
}

console.log(arrayConversion([1, 2, 3, 4, 5, 6, 7, 8]));
