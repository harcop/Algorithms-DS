function almostIncreasingSequence(sequence: number[]): boolean {
    let count = 0;
    sequence.forEach((ele, index) => {
        if (ele <= sequence[index - 1]) {
            count++;
            if (ele <= sequence[index-2] && sequence[index+1] <= sequence[index-1]) {
                console.log(index)
                return false
            }
        }
    })
    console.log(count)
    return count <= 1;
}

console.log(almostIncreasingSequence([1, 2, 1, 1])) 
console.log(almostIncreasingSequence([1, 3, 2])) 