
const arr = [9, 8, 7, 13, 3, 4, 2, 10, 5, 6, 1];

function Heap () {
    this.init = (arr) => {
        this.heap = this.build(arr);
        return this.heap;
    }

    this.build = (arr) => {
        let firstParentIdx = Math.floor((arr.length - 2) / 2);
        console.log(firstParentIdx)
        for (let currentIdx = 0; currentIdx<=firstParentIdx; currentIdx++) {
            console.log(currentIdx)
            this.moveDown(currentIdx, arr.length-1, arr);
        }
        console.log(arr);
        return arr;
    }

    this.moveDown = (currentIdx, endIdx, heap) => {
        childOneIdx = currentIdx * 2 + 1;
        let idxToSwap = childOneIdx;
        console.log(currentIdx, heap[currentIdx])
        while (childOneIdx <= endIdx) {
            console.log('am')
            let childTwoIdx = currentIdx * 2 + 2 <= endIdx ? currentIdx * 2 + 2 ¯: -1;
            console.log(heap[currentIdx], currentIdx)
            console.log(heap[childOneIdx], childOneIdx)
            console.log(heap[childTwoIdx], childTwoIdx)
            if(childOneIdx !== -1 && heap[childOneIdx] > heap[childTwoIdx]) {
                idxToSwap = childTwoIdx;
            }
            console.log(idxToSwap, heap[idxToSwap]);
            if(heap[currentIdx] > heap[idxToSwap]) {
                this.swap(currentIdx, idxToSwap, heap);
                console.log(currentIdx, idxToSwap)
                this.moveUp(currentIdx, heap);
                currentIdx = idxToSwap;
                console.log(currentIdx, idxToSwap, heap[currentIdx])
                childOneIdx = currentIdx * 2 + 1;
                idxToSwap = childOneIdx;
            } else {
                break;
            }
        }
        console.log(heap)
    }

    this.moveUp = (currentIdx, heap) => {
        let parentIdx = Math.floor((currentIdx - 1) / 2);
        console.log(heap[parentIdx])
        while(currentIdx > 0 && heap[parentIdx] > heap[currentIdx]) {
            this.swap(currentIdx, parentIdx, heap);
            currentIdx = parentIdx;
            parentIdx = Math.floor((currentIdx - 1) / 2);
        }
    }

    this.remove = () => {
        let heap = this.heap;
        this.swap(0, heap.length-1, heap);
        let rm = heap.splice(0,1);
        rm
        this.moveDown(0, heap.length-1, heap);
        return heap;
    }

    this.insert = (value) => {
        this.heap.push(value);
        this.moveUp(this.heap.length - 1, this.heap);
        return this.heap;
    }

    this.peak = () => {
        return this.heap[0];
    }

    this.swap = (i, j, heap) => {
        [heap[i], heap[j]] = [heap[j], heap[i]];
    }
}

const heaper = new Heap();
console.log(heaper.init(arr));
console.log(heaper.remove());
