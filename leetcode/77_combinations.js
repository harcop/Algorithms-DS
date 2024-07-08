function combine(n, k) {
    const result = [];

    function backtrack(start, combination) {
        // If the combination is done
        if (combination.length === k) {
            result.push([...combination]);
            return;
        }

        // Go through all possible candidates
        for (let i = start; i <= n; i++) {
            // Add i into the current combination
            combination.push(i);
            // Use next integers to complete the combination
            backtrack(i + 1, combination);
            // Backtrack by removing the last added number
            combination.pop();
        }
    }

    backtrack(1, []);
    return result;
}

console.log(combine(4, 2))
