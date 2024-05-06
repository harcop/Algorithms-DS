/**
 * @param {number[][]} intervals
 * @param {number[]} newInterval
 * @return {number[][]}
 */
var insert = function(intervals, newInterval) {
    intervals.push(newInterval);
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

console.log(insert([[1,3],[6,9]], [2,5]))
console.log(insert([[1,2],[3,5],[6,7],[8,10],[12,16]], [4,8]))
