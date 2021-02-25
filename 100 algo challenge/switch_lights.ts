import { array } from "yargs";

function switchLights(a: number[]): number[] {
    for(let i =0; i < a.length; i++) {
        if (a[i] === 1) {
            a[i] = 0
            for (let j = i-1; j>= 0; j--) {
                if (a[j] === 1) {
                    a[j] = 0;
                    continue;
                }
                a[j] = 1
            }
        }
    }
    return a;
}


console.log(switchLights([1, 1, 1, 1, 1]));
console.log(switchLights([0, 0]));