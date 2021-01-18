const { exec } = require('child_process');
const util = require('util');

function rain() {
    console.log('am here');
    if (this.drop === undefined) {
        async function drop() {
            exec('echo "const fs = require(\'fs\'); const main=fs.readFileSync(\'./zcode.js\').toString(); console.log(main)" >> testRepo.js', (err, stdout, stderr) => {
                exec('node testRepo', (err, stdout, stderr) => {
                    if (err) {
                        console.log('res', err);
                    }
                    console.log('res', stdout);
                })
            })
        };
        this.drop = drop;
        const dd = util.callbackify(drop)
        dd((err, res) => {
            console.log(res);
        })
    }
}
function runner() {
    if (this.runner === undefined) {
        exec('echo "cat zcode.js" >> zt.sh', (err, stdout, stderr) => {
            exec('sh zt.sh', (err, stdout, stderr) => {
                console.log('res', stdout);
            })
        })
    }
    this.runner = "true";
}
function earth () {
    for(let i = 0; i < 15; i++) {
        runner();
    }
}
earth();