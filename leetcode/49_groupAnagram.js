const groupAnagrams = function(strs) {
    const ana = {}
    for(const str of strs) {
      console.log(str)
        const sortedStr = str.split('').sort();
        if(!ana[sortedStr]) {
            ana[sortedStr] = []
        }
        ana[sortedStr].push(str)
    }
    return Object.values(ana)
};

console.log(groupAnagrams(["eat","tea","tan","ate","nat","bat"]))
