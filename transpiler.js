function transpiler(code) {
    code = code.replace("sorosoke", "console");
    code = code.replace("werey", "logs");
    return code;
}
module.exports = {
    transpiler
}