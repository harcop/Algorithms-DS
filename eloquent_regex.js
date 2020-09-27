let reg = /ca(r|t)/ui
console.log(reg.test('car'))
let reg2 = /pr?op/ui
console.log(reg2.test('prop'))
let reg3 = /ferr[a-z]+/ui
console.log(reg3.test('ferr'))
let reg4 = /\w+(ious)$/ui
console.log(reg4.test('ferrious'))
let reg5 = /\s([.,:;])/ui
console.log(reg5.test(' '))
let reg6 = /\w{6,}/ui
console.log(reg6.test('bomber'))
let reg7 = /^\b([^e])+\b$/gi
console.log(reg7.test('fdfsdf'))
let reg8 = /^(')|(')$/gi;
let str8 = "'I love you, no I don't'"
console.log(str8.replace(reg8, '"'));
let reg9 = /^(\.|\+|\-)*\d(e\+?\-?)*\d|\.$/gi
console.log(reg9.test('45'))