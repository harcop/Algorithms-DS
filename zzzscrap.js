const body = require('./zzztest')
// const body = require('./zzold')
const questions = [];
const fs = require('fs');
const path = require('path');

// const writeStream = fs.createWriteStream(path.resolve(__dirname + '/zyxquestions.js'), { encoding: 'utf8', highWaterMark: 16 *
// 1024 });

function extract(body) {
    const split1 = body.split('<section')[1];
    const split2 = split1.split('</section')[0];
    const split3 = split2.split('<div class="question">');
    split3.shift();
    // console.log(split3);
    for (let k = 0; k<split3.length; k++) {
        const question = split3[k];
        // console.log(question);
        const split4 = question.trim().split('<div class="row question-images-wrapper">'); // maybe into 2 if solution exists
        // console.log(split4);
        
        //select the first one
        const questionNumber = split4[0].trim().split('<span class="question-number">')[1].trim().split('</span>')[0].trim();
        // console.log(questionNumber)
        const split5 = split4[1].trim().split('</p></p></p></p></p></p></p></p></p>\n</div>');
        // console.log(split5);
        const questionBody = split5[0].trim().split('</div>')[1].trim();
        // console.log(questionBody);
        const options = [];
        for(let i = 1; i<split5.length-1; i++) {
            const ele = split5[i];
            const split6 = ele.trim().split('<div class="question-option-wrapper">');
            // console.log(split6);
            const split7 = split6[1].trim().split('<span class="question-option">');
            const split8 = split7[1].trim().split('</span>');
            const alphabet = split8[0];
            const optionBody = split8[1].trim();
            options[`${alphabet}`] = optionBody;
        }
        const findAnswer = split5[split5.length-1];
        const split9 = findAnswer.trim().split('<p><strong>Correct Option:</strong>');
        const answer = split9[1].trim().split('</p>')[0]
        const _question = {
            questionNumber,
            questionBody,
            ...options,
            answer
        }
        if (split4[2]) {
            // console.log(split4[2]);
            const split4b = split4[2].trim().split('</p></p></p></p></p></p></p></p></p></p>')[0].trim().split('</div>')[1].trim();
            _question.solution = split4b;
        }
        questions.push(_question);
    }
    // const readStream = fs.createReadStream(JSON.stringify(questions));
    fs.writeFileSync('./zyxquestions.js', JSON.stringify(questions));
    // readStream.pipe(writeStream);
}

extract(body);

// console.log(questions);