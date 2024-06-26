/**
 * @param {string[]} words
 * @param {number} maxWidth
 * @return {string[]}
 */
var fullJustify = function(words, maxWidth) {
    const rows = [];
    let rowWord = '';
    for(const word of words) {
        let temp = (rowWord + ' ' + word).trim()
        if(temp.length > maxWidth) {
            rows.push(rowWord)
            rowWord = word
            continue
        }
        rowWord = temp
    }
    if(rowWord.length) {
      rows.push(rowWord);
    }

    let newRow = []
    
    for(let r = 0; r < rows.length; r++) {
      let row = rows[r]
      if(row.length === maxWidth) {
        newRow.push(row)
        continue
      }
      let rowArr = row.split(" "); // 3
      
      let spaces = rowArr.length - 1; // 2
      
      if(spaces === 0 || r === rows.length - 1) {
        newRow.push(row.padEnd(maxWidth, " "))
        continue
      }

      let reminderSpace = maxWidth - row.length + spaces; // 8
      let share = Math.floor(reminderSpace / spaces) // 4
      let leftover = reminderSpace - (spaces * share) // more empty slot to the right

      let i = spaces
      let j = 0;
      while(i > 0) {
        let textLength = share + (leftover > 0 ? 1 : 0) + rowArr[j].length
        rowArr[j] = rowArr[j].padEnd(textLength)
        if(leftover > 0) {
          leftover--
        }
        j++
        i--
      }
      newRow.push(rowArr.join(""))
    }

    return newRow
};

console.log(fullJustify(["This", "is", "an", "example", "of", "text", "justification."], 16))
console.log(fullJustify(["What","must","be","acknowledgment","shall","be"], 16))
console.log(fullJustify(["Science","is","what","we","understand","well","enough","to","explain","to","a","computer.","Art","is","everything","else","we","do"], 20))
