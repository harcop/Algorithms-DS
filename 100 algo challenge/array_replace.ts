function arrayReplace(arr: number[], elemToReplace: number, substitutionElem: number): number[] {
    arr.forEach((ele, index) => {
        if (ele === elemToReplace) {
         arr[index] = substitutionElem;
        }
    })
    return arr;
 }
 
 console.log(arrayReplace([1, 2, 1], 1, 3));