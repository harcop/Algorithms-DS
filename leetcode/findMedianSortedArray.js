var findMedianSortedArrays = function(nums1, nums2) {
    let notFinished = true;
    let newArray = []
    while(notFinished) {
        console.log(nums1, nums2)
        if(!nums1.length && !nums2.length) {
            notFinished = false
        }
        if(!nums1.length) {
            newArray.push(...nums2)
            notFinished = false
        }
        else if(!nums2.length) {
            newArray.push(...nums1)
            notFinished = false
        }
        else {
            if(nums1[0] < nums2[0]) {
                newArray.push(...nums1.splice(0, 1))
            } else {
                newArray.push(...nums2.splice(0, 1))
            }
        }
    }
    if(newArray.length % 2 === 1) {
        return newArray[Math.floor(newArray.length/2)]
    }
    const centerNumber = Math.ceil(newArray.length/2)
    return (newArray[centerNumber] + newArray[centerNumber - 1]) / 2
};

console.log(findMedianSortedArrays([1,2], [3, 4]))
