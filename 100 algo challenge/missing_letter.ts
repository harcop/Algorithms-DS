function missingLetters(str: string): string {
   let _s = str.split("");
   let _c = _s[0].charCodeAt(0);
   for(let i = 1; i < _s.length; i++) {
       let _cc = _s[i].charCodeAt(0);
       let _d = _cc - _c;
       if (_d > 1) {
           return String.fromCharCode(_c+1);
       }
       _c = _cc;
   }
}
console.log(missingLetters("bce"));
console.log(missingLetters("abce"));
console.log(missingLetters("abcdefghjklmno"));
console.log(missingLetters("abcdefghijklmnopqrstuvwxyz"));