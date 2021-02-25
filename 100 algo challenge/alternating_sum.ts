function alternatingSums(a: number[]): number[] {
    const output = [0,0];
    a.forEach((ele, index) => {
        if (index % 2 === 1) {
            output[1] += ele
        }
        else {
            output[0] += ele
        }
    })
    return output;
}

console.log(alternatingSums([50, 60, 60, 45, 70]))