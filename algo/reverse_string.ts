function reverseAString(str: string): string {
        // time O(n) && space O(n);
        let _s = str.split("");
        let _f = [];        
        for(let i =_s.length; i>=0; i--) {
            _f.push(_s[i]);
        }
        return _f.join("");

        //time O(log(n)) && space O(n); which is better than above
        // let lp = 0;
        // let rp = _s.length-1;
        // while(lp < rp) {
        //     [_s[lp], _s[rp]] = [_s[rp], _s[lp]];
        //     rp--;
        //     lp++;
        // }
        // return _s.join("");;
}

console.log(reverseAString('hello'));
console.log(reverseAString('Howdy'));