
const width = 21;
const height = 21;
$('document').ready(function() {
    setup()
    border()
    split()
})


function setup() {
        for(let i = 1; i <= height; i++) {
        for(let j = 1; j <= width; j++) {
            $('.main').append(`
                    <div class="box ${i}-${j}"></<div>
                `);
        }
        $('.main').append(`<br>`);
    }
}

function border() {
    for(let i = 1; i <= height; i++) {
    for(let j = 1; j <= width; j++) {
        if (i === 1 || j === 1 || i === height || j === width) {
            $(`.${i}-${j}`).addClass("borderGate")
        }
    }}
}


const ws = [{sw: 1, ew: width, sh: 1, eh: height}];
let side = 1;
function split() {
    while (ws.length) {
        const currentD = ws[0];
        const { sw, ew, sh , eh} = currentD;

        let pw = Math.floor(Math.random() * (ew-3)) + 2; //picked width
        if (!(pw % 2)) {
            pw += 1;
        }

        let n = Math.floor(Math.random() * 17) + 3;
        if (n%2) {
            n += 1;
        }

        if (side) {
            const diff = eh - sh;
            if (diff >= 4) {
                for(let i = sh; i < eh; i++) {
                    $(`.${i}-${pw}`).addClass("borderGate");
                }
                $(`.${n}-${pw}`).removeClass("borderGate");
            }
        } else {
            const diff = ew - sw;
            if (diff >= 4) {
                for(let i = sw; i < ew; i++) {
                    $(`.${pw}-${i}`).addClass("borderGate")
                }
                $(`.${pw}-${n}`).removeClass("borderGate");
            }
        }
        console.log(n, 'number', pw);

        const _n1sw = sw;
        console.log(pw, 'picked point');
    }
    side = 0;
//    split();
}