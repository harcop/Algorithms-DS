const arr = [1,5,10,25];
const amt = 15;

//time O(Nd) && space O(N);
function nw(arr, amt) {
    const l = Array(amt+1);
    l.fill(0,0,amt+1);
    l[0] = 1;
    arr.forEach((ele) => {
        for(let j = ele; j<l.length; j++) {
            l[j] += l[j-ele];
        }
    })
    return l[amt];
}

console.log(nw(arr, amt));