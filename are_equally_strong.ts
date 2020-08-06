function areEquallyStrong(yourLeft: number, yourRight: number, friendsLeft: number, friendsRight: number): boolean {
    let eqs = 0;
    if (yourLeft === friendsLeft || yourLeft === friendsRight) {
        eqs += 1;
    }
    if (yourRight === friendsRight || yourRight === friendsLeft) {
        eqs += 1;
    }
    return eqs == 2;
}

console.log(areEquallyStrong(10, 15, 15, 10))
console.log(areEquallyStrong(15, 10, 15, 10))
console.log(areEquallyStrong(15, 10, 15, 9))
