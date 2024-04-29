function isAvailable(meetings, mystartTime, myEndTime) {
  meetings.unshift([0, 0])
  meetings.push([Infinity, Infinity])

 for(let i = 1; i < meetings.length - 1; i++) {
    const meeting = meetings[i]
    const [_, meetingEnd] = meeting;
    if(mystartTime >= meetingEnd && myEndTime <= meetings[i + 1][0]) {
      return true
    }
 }
 return false
}
// 845, 900, 1230, 1300, 1300, 1500
const meetings = [[845, 900], [1230, 1300], [1300, 1500]]

console.log(isAvailable(meetings, 900, 1230))  // => true
console.log(isAvailable(meetings, 915, 1215))  // => true
console.log(isAvailable(meetings, 850, 1240))  // => false
console.log(isAvailable(meetings, 1200, 1300)) // => false
console.log(isAvailable(meetings, 700, 1600))  // => false
console.log(isAvailable(meetings, 800, 845))   // => true
console.log(isAvailable(meetings, 1500, 1800)) // => true
console.log(isAvailable(meetings, 845, 859))   // => false
console.log(isAvailable(meetings, 846, 900))   // => false
console.log(isAvailable(meetings, 846, 859))   // => false
console.log(isAvailable(meetings, 845, 900))   // => false
console.log(isAvailable(meetings, 2359, 2400)) // => true
console.log(isAvailable(meetings, 930, 1600))  // => false
console.log(isAvailable(meetings, 800, 850))   // => false
console.log(isAvailable(meetings, 1400, 1600)) // => false
console.log(isAvailable(meetings, 1300, 1501)) // => false
