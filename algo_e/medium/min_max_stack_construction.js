function Mmsc () {
    this.minMax = [];
    this.stack = [];

    this.insert = (number) => {
        let newMinMax = { min: number, max: number };
        if (this.minMax.length) {
            let lmin = this.minMax[this.minMax.length -1]['min'];
            let lmax = this.minMax[this.minMax.length -1]['max'];
            newMinMax['min'] = Math.min(lmin, number);
            newMinMax['max'] = Math.max(lmax, number);
        }
        this.minMax.push(newMinMax);
        this.stack.push(number);
        return this;
    }

    this.peak = () => {
        return this.stack[this.stack.length -1];
    }

    this.pop = () => {
        let _pp = this.stack.pop();
        this.minMax.pop();
        return _pp
    }

    this.getMin = () => {
        return this.stack[this.stack.length -1]['min'];
    }

    this.getMin = () => {
        return this.stack[this.stack.length -1]['max'];
    }
}

const mm = new Mmsc();
console.log(mm.insert('3').insert(4));