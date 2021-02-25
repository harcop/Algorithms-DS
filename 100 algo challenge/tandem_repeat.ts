function isTandemRepeat(str: string): boolean {
    let _l = Math.ceil(str.length/2);
    let _s1 = str.slice(0,_l);
    let _s2 = str.slice(_l);
    return _s1===_s2;
}  
console.log(isTandemRepeat('tandemtandem'))
console.log(isTandemRepeat('qqq'))
console.log(isTandemRepeat('2w2w2w'))
