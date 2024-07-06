function minWindow(word, sub) {
    if (word.length === 0 || sub.length === 0) {
        return "";
    }

    const subFreq = {}
    const windowFreq = {}

    // Populate frequency map for sub
    for (let ele of sub) {
        subFreq[ele] = (subFreq[ele] || 0) + 1;
    }

    console.log(subFreq)

    let have = 0;
    const need = Object.keys(subFreq).length;
    let result = [-1, -1];
    let minLength = Infinity;
    let left = 0;

    // Expand the window by moving the right pointer
    for (let right = 0; right < word.length; right++) {
        let char = word[right];
        windowFreq[char] = (windowFreq[char] || 0) + 1;

        if (subFreq[char] && windowFreq[char] === subFreq[char]) {
            have++;
        }

        // When we have a valid window, try to contract it by moving the left pointer
        while (have === need) {
            // Update result if this window is smaller
            if ((right - left + 1) < minLength) {
                result = [left, right];
                minLength = right - left + 1;
            }

            // Remove leftmost character from the window
            let leftChar = word[left];
            windowFreq[leftChar] = windowFreq[leftChar] - 1;
            if (subFreq[leftChar] && windowFreq[leftChar] < subFreq[leftChar]) {
                have--;
            }
            left++;
        }
    }

    const [start, end] = result;
    return minLength === Infinity ? "" : word.slice(start, end + 1);
}


console.log(minWindow("ADOBECODEBANC", "ABC"))
