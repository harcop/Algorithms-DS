// a test code for node server exploitations 
const { exec } = require('child_process');

exec('echo "const fs = require(\'fs\'); const main=fs.readFileSync(\'./zcode.js\').toString(); console.log(main)" >> testRepo.js', (err, stdout, stderr) => {
    exec('node testRepo', (err, stdout, stderr) => {
        console.log('goal', stdout);
    })
})