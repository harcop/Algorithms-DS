const { qs } = require('./quick_sort.js');
const arr = [8,5,2,9,5,6,3];
const k = 3;
const r = qs(arr, 0, arr.length);
console.log(r[k-1]);