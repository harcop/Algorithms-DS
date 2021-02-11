//MCVII = 1107

function converter (roman) {
    const _roman = roman.split('');
    let number = 0;
    for(let i = 0; i < _roman.length; i++) {
        let _r = _roman[i];
        if (digit(_r) < 0) {
            console.log('invalid roman');
            return 0;
        }
        if (_roman.length - i > 2) {
            if (digit(_r) < digit(_roman[i+2])) {
                console.log('invalid roman');
                return 0;
            }
        } 
        if (digit(_r) >= digit(_roman[i+1])) {
            number += digit(_r);
        } else {
            number += (digit(_roman[i+1]) - digit(_r));
        }
    }
    return number;
}

function digit(val) {
    const roman = {
        I: 1,
        V: 5,
        X: 10,
        L: 50,
        C: 100,
        M: 1000
    }
    if (!Object.keys(roman).includes(val)) {
        return -1;
    }
    return roman[val];
}


console.log(converter('MCVII'))
console.log(converter('X'))