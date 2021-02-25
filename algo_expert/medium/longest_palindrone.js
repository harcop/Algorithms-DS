const arr = ["a", "b", "a", "x", "y", "z", "d", "z", "y", "x", "b"];
// abba
// abdcdba
function  lp (arr) {
    let ll = [[]];
    for(let i=0; i<arr.length; i++) {
       let s1 = i-1;
       let s2 = i+1;
       if (arr[s1] === arr[s2]) {
            tt(s1, s2, arr, ll);
       } else if (arr[s1] === arr[i]) {
            console.log(arr[s1], arr[i])
            s2 = i;
            tt(s1, s2, arr, ll);
       }
    }
    return ll;
}

function tt (s1, s2, arr, ll) {
    s1 -= 1;
    s2 += 1;
    while (arr[s1] === arr[s2]) {
        s1 -= 1;
        s2 += 1;
    }
    let na = arr.slice(s1+1, s2);
    if (na.length > ll[0].length) {
        ll[0] = na;
    }
}

console.log(lp(arr));