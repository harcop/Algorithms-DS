const arr = [5,7,3,2,6]
const k = 10;

function two_sum_num(arr, k) {
    arr.sort((a,b) => a-b);
    let lp = 0;
    let rp = arr.length-1;
    console.log(arr);
    while (lp < rp) {
        let _s = arr[lp] + arr[rp];
        if (_s > k) {
            rp--;
        }else if (_s < k) {
            lp++;
        }
        else {
            console.log(arr[lp], arr[rp])
            return [arr[lp], arr[rp]];
        }
    }
    return false;
}
console.log(two_sum_num(arr, k));

function three_sum_num (arr, k) {
    arr.sort((a, b) => a-b);
    console.log(arr);
    for(let i = 0; i< arr.length; i++) {
        let c = arr[i];
        let lp = 1;
        let rp = arr.length-1;
        while (lp < rp) {
            let _s = arr[lp] + arr[rp] + c;
            if (_s > k) {
                rp--;
            }
            else if (_s < k) {
                lp++;
            }
            else {
                return [c, arr[lp], arr[rp]]
            }
        }
    }
    return false;
}

console.log(three_sum_num(arr,k))


function four_sum_num (arr, k) {
    arr.sort((a, b) => a-b);
    console.log(arr);
    for (let j = 0; j< arr.length; j++) {
        let c1 = arr[j];
        for(let i = j+1; i< arr.length; i++) {
            let c = arr[i];
            let lp = i+1;
            let rp = arr.length-1;
            while (lp < rp) {
                let _s = arr[lp] + arr[rp] + c + c1;
                if (_s > k) {
                    rp--;
                }
                else if (_s < k) {
                    lp++;
                }
                else {
                    return [c1, c, arr[lp], arr[rp]]
                }
            }
        }
    }
    return false;
}

console.log(four_sum_num(arr,18))