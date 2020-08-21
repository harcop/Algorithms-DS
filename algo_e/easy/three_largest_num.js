
// naive method
const arr = [141, 1, 17, -7, -17, -27, 18, 541, 8, 7, 7];
// arr.sort((a,b) => a- b);
// const newArr = arr.slice(arr.length-3);
// console.log(newArr);

const _f = [0, 0, 0];
// function ls(arr) {
//     arr.forEach(ele => {
//         if (ele > _f[2]) {
//             _f[0] = _f[1];
//             _f[1] = _f[2]  
//             _f[2] = ele;
//         }
//         else if (ele > _f[1]) {
//             _f[0] = _f[1];
//             _f[1] = ele;
//         }
//         else if (ele > _f[0]) {
//             _f[0] = ele;
//         }
//     })
//     return _f
// }


console.log(ls(arr));