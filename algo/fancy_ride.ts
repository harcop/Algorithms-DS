function fancyRide(l: number, f: number[]): string {
    let m = 20;
    let r = 0;
    let c = ["UberX", "UberXL", "UberPlus", "UberBlack", "UberSUV"]
    for(let i = 0; i<f.length; i++) {
        if (f[i] * l > m) {
            r = i-1
            break;
        }
    }
    return c[r];
}

console.log(fancyRide(30, [0.3, 0.5, 0.7, 1, 1.3]));