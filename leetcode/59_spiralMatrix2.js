var generateMatrix = function(n) { 
    // n = 3
    // create empty 3 x 3 array
    const result = new Array(n).fill(0).map(x => []);
    console.log(result)


    let counter = 0;
    let first = 0;
    let last = n-1;

    // 0 -> till ceil(3/2) = 2 = 3 loops
    for (let i = 0; i < Math.ceil(n/2); i++) {
      // 0 -> 2
        for(let j = first; j <= last; j++)
            // [[1, 2, 3], [], []]
            result[first][j] = ++counter

            // 1 -> 2
        for(let j = first+1; j <= last; j++)
          // [[1, 2, 3], [0,0,4], [0,0,5]]
            result[j][last] = ++counter

            // 1 -> 0
        for(let j = last-1; j >= first; j--)
          // [[1, 2, 3], [0,0,4], [7,6,5]]
            result[last][j] = ++counter

            // 1 -> 0
        for(let j = last-1; j >= first+1; j--)
          // [[1, 2, 3], [8,9,4], [7,6,5]]
            result[j][first] = ++counter

        first++
        last--
    }

    return result;
};

console.log(generateMatrix(3))
console.log(generateMatrix(4))
