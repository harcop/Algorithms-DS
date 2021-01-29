const fs = require('fs');
const { transpiler } = require('./transpiler');
const { exec } = require('child_process');

let code = `function printer() { 
\t print.out('hey this is a word'); 
}
printer();`;
code = transpiler(code);
fs.writeFileSync("./exec/code.js", code);

exec("node ./exec/code.js", (error, stdout, stderr) => {
    if (error) {
        console.log(`error: ${error.message}`);
        return;
    }
    if (stderr) {
        console.log(`stderr: ${stderr}`);
        return;
    }
    console.log(`stdout: ${stdout}`);
})