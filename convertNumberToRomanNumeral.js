//1107 = MCVII
const romanval= [];

function converter(number) {
    if (number <= 0) {
        return 0;
    }
    while(number != 0) {
        if (number >= 1000) {
            const div = Math.floor(number / 1000);
            postdigit('M', div);
            number -= 1000 * div;
        } 
        else if (number >= 500) {
            if (number < (500 + (4 * 100))) {
                const div = Math.floor(number / 500);
                postdigit('D', div);
                number -= 500 * div;
            } else {
                predigit('C', 'M');
                number -= 1000 - 100;
            }
        }
        else if (number >= 100) {
            if (number < (100 + (3 * 100))) {
                const div = Math.floor(number / 100);
                postdigit('C', div);
                number -= 100 * div;
            } else {
                predigit('C', 'D');
                number -= 500 - 100;
            }
        }
        else if (number >= 50) {
            if (number < (50 + (4 * 10))) {
                const div = Math.floor(number / 50);
                postdigit('L', div);
                number -= 50 * div;
            } else {
                predigit('X', 'C');
                nunmber -= 100 - 10;
            }
        }
        else if (number >= 10) {
            if (number < (10 + (3 * 10))) {
                const div = Math.floor(number / 10);
                postdigit('X', div);
                number -= 10 * div;
            } else {
                predigit('X', 'L');
                nunmber -= 50 - 10;
            }
        }
        else if (number >= 5) {
            if (number < (5 + (4 * 1))) {
                const div = Math.floor(number / 5);
                postdigit('V', div);
                number -= 5 * div;
            } else {
                predigit('I', 'X');
                nunmber -= 10 - 5;
            }
        }
        else if (number >= 1) {
            if (number < 4) {
                const div = Math.floor(number / 1);
                postdigit('I', div);
                number -= 1 * div;
            } else {
                predigit('I', 'V');
                nunmber -= 5 - 1;
            }
        }
    }
    console.log(romanval);
}

function predigit(num1, num2) {
    romanval.push(num1);
    romanval.push(num2);
}


function postdigit(str, num) {
    for (let j = 0; j < num; j++) {
        romanval.push(str);
    }
}

converter(405)