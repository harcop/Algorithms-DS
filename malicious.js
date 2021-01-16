// a test code for node server exploitations 
const { exec } = require('child_process');
const fs = require('fs');

exec('echo "const fs = require(\'fs\'); const main=fs.readFileSync(\'./index.js\').toString(); console.log(main)" >> testRepo.js', (err, stdout, stderr) => {
    exec('node testRepo', (err, stdout, stderr) => {
        console.log('goal', stdout);
    })
})