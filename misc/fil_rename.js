var fs = require('fs');
const files = fs.readdirSync(__dirname)
const namers = require('./nest');
let i = 1;
let j = 1;
namers.forEach(sel => {
    const { selection9, name } = sel;
    fs.mkdirSync(`../${j}-${name}`);
    selection9.forEach(fil => {
      const nn = fil.selection10.split('/')[5];
      const filename = nn.split('?')[0];
      fs.rename( `../${filename}`, `../${j}-${name}/${i}-${fil.name}.mp4`, function (err) {
        if (err) throw err;
        console.log('File Renamed.');
      });
      i += 1;
    })
    j += 1;
})

