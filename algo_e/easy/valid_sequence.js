function isValidSubsequence(array, sequence) {
  // Write your code here.
	let ad = 0;
	let sd = 0;
	for (const _s of sequence) {
		const index = sequence.indexOf(_s);
		if (array.includes(_s)) {
			const index2 = array.indexOf(_s);
			if (index < sd || index2 < ad) {
				return false;
			}
			array.splice(index2, 1)
			sd = index;
			ad = index2
		} else {
			return false;
		}
	}
	return true;
}