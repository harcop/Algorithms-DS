const fs = require('fs');
const { transpiler } = require('./transpiler');
const { exec } = require('child_process');

let code = `function namer() { 
    sorosoke.werey('endsar'); 
} 
namer();`;
code = transpiler(code);
fs.writeFileSync("./zcode.js", code);

exec("node zcode", (error, stderr, stdout) => {
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