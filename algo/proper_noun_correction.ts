function properNounCorrection(n: string): string {
    let _n = n.toLowerCase();
    _n = _n[0].toUpperCase() + _n.slice(1);
    return _n;
    }
    
    console.log(properNounCorrection('pARiS'));
    console.log(properNounCorrection('John'));