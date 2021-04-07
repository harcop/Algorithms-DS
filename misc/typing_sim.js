let messagingDays = '';
const days = ['Holidays', 'Birthday', 'New Month', 'Special Days']

function inc() {
let d = 0;
 let j = 0
 let k = 1
 let text = days[d];
 const tt = setInterval(() => {
   if (j < text.length) {
     dis(text[j]);
    } else {
     dec();
     k += 1;
   }
   j += 1;
   if (j === text.length * 2) {
     if (d === days.length - 1) {
       d = -1;
     }
     text = days[++d];
     j = 0;
     k = 1;
    // clearInterval(tt);
   }
 }, 500)
}

function dis(text) {
  messagingDays += text;
  console.log(messagingDays);
}
function dec() {
  messagingDays = messagingDays.substring(0, messagingDays.length-1);
  console.log(messagingDays);
}

function disDec(text) {
    this.messagingDays = text.substring(0, text.length-1);
}

inc()
