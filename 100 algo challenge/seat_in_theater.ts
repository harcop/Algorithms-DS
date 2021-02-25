function seatsInTheater(nc: number, nr: number, c: number, r: number): number {
    let _c = nc - c + 1;
    let _r = nr - r;
    return _c * _r;
}

console.log(seatsInTheater(16, 11, 5, 3));