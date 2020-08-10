function characterParity(sym: string): string {
    let intr = parseInt(sym);
    console.log(intr)
    if (intr%2 === 0) {
        return 'even'
    }
    else if(intr%2 === 1) {
        return 'odd'
    }
    return 'not a digit';
 }
 
 console.log(characterParity('5'))
 console.log(characterParity('8'))
 console.log(characterParity('q'))
 