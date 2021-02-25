function integerToStringOfFixedWidth(n: number, w: number): string {
    if (n.toString().length === w) {
        return n.toString();
    }
    else if (n.toString().length < w) {
        let _d = w - n.toString().length;
        console.log(_d);
        let _z = "0".repeat(_d);
        return `${_z}${n}`;
    }
    else if (n.toString().length > w) {
        let _d = n.toString().substr(n.toString().length-w)
        return `${_d}`;
    }
}

console.log(integerToStringOfFixedWidth(1234, 2));
console.log(integerToStringOfFixedWidth(1234, 4));
console.log(integerToStringOfFixedWidth(1234, 5));
