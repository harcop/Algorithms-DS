function areSimilar(a: number[], b: number[]): boolean {
    a.sort((a,b) => a-b);
    b.sort((a,b) => a-b);
    for (let i = 0; i < a.length; i++) {
       if (a[i] !== b[i]) {
          return false;
       }
    }
    return true;
    // for (let i = 0; i < a.length; i++) {
    //    let ele = a[i];
    //    if (!b.includes(ele)) {
    //       console.log(ele)
    //       return false
    //    }
    //    b.splice(b.indexOf(ele), 1);
    // };
    // console.log(b)
    // return true;
}

console.log(areSimilar([1, 2, 3], [1, 2, 3]));
console.log(areSimilar([1, 2, 3], [2, 1, 3]));
console.log(areSimilar([1, 2, 2], [2, 1, 1]));
