const value = 10;
const arr = [3, 5, -4, 8, 11, 1, -1, 6];

function two_sum(arr, value) {

    for (let i=0; i < arr.length; i++) {
        const a = arr[i];
        for (let j=i+1; j<arr.length; j++) {
            const b = arr[j];
            if((a + b) === value) {
                console.log(a, b);
                return [a, b];
            }
        }
    }   
    return [];
}

// function two_sum (arr, value, hash = {}) {
//     for (const ele of arr) {
//         const alt = Math.floor(value - ele);
//         if(hash[alt]) {
//             console.log(alt);
//             return `${ele} + ${alt} = ${value}`;
//         }
//         hash[ele] = ele;
//         console.log(hash[ele]);
//     };
// }

//we need to sort the array first;
// function two_sum(arr, value) {
//     arr.sort((a, b) => a - b);
//     console.log(arr);
//     let left = 0;
//     let right = arr.length - 1;
//     while(left < right) {
//         let currentSum = arr[left] + arr[right];
//         console.log(currentSum);
//         if(currentSum < value) {
//             left += 1;
//         }
//         else if (currentSum > value) {
//             right -= 1;
//         }
//         else if (currentSum === value) {
//             return [arr[left], arr[right]];
//         }
//     }
//     return []
// }

