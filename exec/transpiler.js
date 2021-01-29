module.exports.transpiler = (code) => {
    const _code = code.replace('print.out', 'console.log')
    return _code;
}   