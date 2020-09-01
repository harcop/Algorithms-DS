function minimalNumberOfCoins(c: number[], p: number): number {
    //how many time you can generate price from coins
    // const _a = Array(p+1);
    // const _p = _a.fill(0,0,p+1);
    // _p[0] = 1;
    // c.forEach(ele  => {
    //     if (ele > p) return;
    //     for(let i = ele; i<_p.length; i++) {
    //         let _d = i - ele;
    //         _p[i] += _p[_d];
    //     }
    // })

    //min coins to you can use to generate price
    let _ms = 0;
    let _df = p;
    for (let i=c.length-1;i>=0;i--) {
        if (c[i] > p) continue;
        let _d = Math.floor(_df/c[i]);
        _df = Math.floor(_df%c[i]);
        console.log(_d,_df);
        _ms += _d;
        if (_df === 0) break;
    }

    return _ms;
}

console.log(minimalNumberOfCoins([1, 2, 10], 28));
console.log(minimalNumberOfCoins([1, 2, 10], 30));