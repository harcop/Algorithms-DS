/**
 * @param {number[][]} intervals
 * @return {number}
 */
var eraseOverlapIntervals = function(intervals) {
    intervals.sort((a, b) => a[1] - b[1]);
    console.log(intervals)
    let overlapCount = 0;
    let [_, prevEnd] = intervals[0];
    for(let i = 1; i < intervals.length; i++) {
        const [currentStart, currentEnd] = intervals[i];
        if(currentStart < prevEnd) {
            overlapCount += 1;
        } else {
            prevEnd = currentEnd;
        }
    }
    return overlapCount;
};

console.log(eraseOverlapIntervals([[1,2],[2,3],[3,4],[1,3]]))
console.log(eraseOverlapIntervals([[1,2],[1,2],[1,2]]))
console.log(eraseOverlapIntervals([[1,2],[2,3]]))
console.log(eraseOverlapIntervals([[-52,31],[-73,-26],[82,97],[-65,-11],[-62,-49],[95,99],[58,95],[-31,49],[66,98],[-63,2],[30,47],[-40,-26]]))
