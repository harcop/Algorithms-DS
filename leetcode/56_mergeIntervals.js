/**
 * @param {number[][]} intervals
 * @return {number[][]}
 */
var merge = function(intervals) {
    intervals.sort((a, b) => a[0] - b[0] || a[1] - b[1])
    let result = [];
    let [min, max] = intervals[0]
    for(let i = 1; i < intervals.length; i++) {
        const [m, x] = intervals[i]
        if(m > max) {
            result.push([min, max])
            min = m;
            max = x;
        } else {
            min = Math.min(min, m);
            max = Math.max(max, x)
        }
    }
    result.push([min, max])
    return result;
};

console.log(merge([[1,3],[2,6],[8,10],[15,18]]))
console.log(merge([[1,4],[4,5]]))
console.log(merge([[1,4],[0,0]]))
