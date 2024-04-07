function findSubstring(s, words) {
  const results = [];
  const wordLength = words[0].length;
  const wordCount = new Map();

  // Build the frequency map for the words
  words.forEach(word => {
    wordCount.set(word, (wordCount.get(word) || 0) + 1);
  });

  for(let i = 0; i < wordLength; i++) { // this is incase the letter are repeating number

  let left = i;
  let right = i;
  let count = 0;
  let seenWords = new Map();

  while (right + wordLength <= s.length) {
    let word = s.slice(right, right + wordLength);
    
    right += wordLength;
    if (wordCount.has(word)) {
      seenWords.set(word, (seenWords.get(word) || 0) + 1);
      count++;

      while (seenWords.get(word) > wordCount.get(word)) {
        let leftWord = s.slice(left, left + wordLength);
        seenWords.set(leftWord, seenWords.get(leftWord) - 1);
        count--;
        left += wordLength;
      }

      if (count === words.length) {
        results.push(left);
        let leftWord = s.slice(left, left + wordLength);
        seenWords.set(leftWord, seenWords.get(leftWord) - 1);
        count--;
        left += wordLength;
      }
    } else {
      // Reset the window if the word is not found in 'words'
      seenWords.clear();
      count = 0;
      left = right;
    }
  }
}
  return results;
}
console.log(findSubstring("aaaaaaaaaaaaaa", ["aa","aa"]))
// console.log(findSubstring("rbarfoothefoobarman", ["bar","foo"]))
// console.log(findSubstring("barfoothefoobarman", ["bar","foo"]))
// console.log(findSubstring("barfoofoobarthefoobarman", ["bar","foo","the"]))

// const ss = "pjzkrkevzztxductzzxmxsvwjkxpvukmfjywwetvfnujhweiybwvvsrfequzkhossmootkmyxgjgfordrpapjuunmqnxxdrqrfgkrsjqbszgiqlcfnrpjlcwdrvbumtotzylshdvccdmsqoadfrpsvnwpizlwszrtyclhgilklydbmfhuywotjmktnwrfvizvnmfvvqfiokkdprznnnjycttprkxpuykhmpchiksyucbmtabiqkisgbhxngmhezrrqvayfsxauampdpxtafniiwfvdufhtwajrbkxtjzqjnfocdhekumttuqwovfjrgulhekcpjszyynadxhnttgmnxkduqmmyhzfnjhducesctufqbumxbamalqudeibljgbspeotkgvddcwgxidaiqcvgwykhbysjzlzfbupkqunuqtraxrlptivshhbihtsigtpipguhbhctcvubnhqipncyxfjebdnjyetnlnvmuxhzsdahkrscewabejifmxombiamxvauuitoltyymsarqcuuoezcbqpdaprxmsrickwpgwpsoplhugbikbkotzrtqkscekkgwjycfnvwfgdzogjzjvpcvixnsqsxacfwndzvrwrycwxrcismdhqapoojegggkocyrdtkzmiekhxoppctytvphjynrhtcvxcobxbcjjivtfjiwmduhzjokkbctweqtigwfhzorjlkpuuliaipbtfldinyetoybvugevwvhhhweejogrghllsouipabfafcxnhukcbtmxzshoyyufjhzadhrelweszbfgwpkzlwxkogyogutscvuhcllphshivnoteztpxsaoaacgxyaztuixhunrowzljqfqrahosheukhahhbiaxqzfmmwcjxountkevsvpbzjnilwpoermxrtlfroqoclexxisrdhvfsindffslyekrzwzqkpeocilatftymodgztjgybtyheqgcpwogdcjlnlesefgvimwbxcbzvaibspdjnrpqtyeilkcspknyylbwndvkffmzuriilxagyerjptbgeqgebiaqnvdubrtxibhvakcyotkfonmseszhczapxdlauexehhaireihxsplgdgmxfvaevrbadbwjbdrkfbbjjkgcztkcbwagtcnrtqryuqixtzhaakjlurnumzyovawrcjiwabuwretmdamfkxrgqgcdgbrdbnugzecbgyxxdqmisaqcyjkqrntxqmdrczxbebemcblftxplafnyoxqimkhcykwamvdsxjezkpgdpvopddptdfbprjustquhlazkjfluxrzopqdstulybnqvyknrchbphcarknnhhovweaqawdyxsqsqahkepluypwrzjegqtdoxfgzdkydeoxvrfhxusrujnmjzqrrlxglcmkiykldbiasnhrjbjekystzilrwkzhontwmehrfsrzfaqrbbxncphbzuuxeteshyrveamjsfiaharkcqxefghgceeixkdgkuboupxnwhnfigpkwnqdvzlydpidcljmflbccarbiegsmweklwngvygbqpescpeichmfidgsjmkvkofvkuehsmkkbocgejoiqcnafvuokelwuqsgkyoekaroptuvekfvmtxtqshcwsztkrzwrpabqrrhnlerxjojemcxel"

// const swords = ["dhvf","sind","ffsl","yekr","zwzq","kpeo","cila","tfty","modg","ztjg","ybty","heqg","cpwo","gdcj","lnle","sefg","vimw","bxcb"]

// console.log(findSubstring(ss, swords))
