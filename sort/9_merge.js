// 54386791
const arr = [5,4,3,8,6,7,9,1];

function mergeSort(arr, l, m, r) {
    let n1 = m - l + 1;
    let n2 = r - m;

    let L = [];
    let R = [];
     for (let i =0; i<n1; i++) {
         L[i] = arr[l + i];
     }
     for(let j =0; j< n2; j++) {
         R[j] = arr[m + 1 + j];
     }

     let i = 0;
     let j = 0;
     let k = l;

     while (i < n1 && j < n2) {
         if (L[i] <= R[j]) {
             arr[k] = L[i];
             i += 1;
         } else {
             arr[k] = R[j];
             j += 1;
         }
         k += 1;
     }

     while (i<n1) {
         arr[k] = L[i];
         i += 1;
         k += 1;
     }

     while (j<n2) {
         arr[k] = R[j];
         j += 1;
         k += 1;
     }
}

function sort(arr, l, r) {
    if (l >= r) {
        return;
    }
    let m = l + (r-l)/2;
    sort(arr, l, m);
    sort(arr, m+1, r);
    mergeSort(arr, l, m, r);
}

function main() {
    sort(arr, 0, arr.length -1);
    // return arr;
}

console.log(main());