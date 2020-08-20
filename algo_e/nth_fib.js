// 1, 1, 2, 3, 5, 8, 13, 21, 34

// time O(2^n) && space O(n)
function getFib2(n) {
    if (n === 1 || n === 2) {
        return 1;
    }
    return getFib(n-1) + getFib(n-2);
}

// time O(n) && space O(n)
function getFib2(n) {
    let m = [1,1];
    if (n === 1 || n === 2) {
        return m[1];
    }
    let i = n;
    while(n>2) {
        m.push(m[m.length-1] + m[m.length-2]);
        console.log(m);
        n--;
    }
    return m[i-1];
}

// time O(n) && space O(1)
function getFib(n) {
    let m = [1,1];
    while(n>2) {
        let temp = m[0] + m[1];
        m[0] = m[1];
        m[1] = temp
        console.log(m);
        n--;
    }
    return m[1];
}
console.log(getFib(4)); 