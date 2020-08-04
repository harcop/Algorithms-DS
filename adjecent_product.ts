function adjacentElementsProducts(ina: number[]): number {
    var lgs = 0;
    ina.forEach((ele, index) => {
        if(ina[index + 1] != undefined) {
            var sml = ele * ina[index+1];
            if(sml > lgs) {
                lgs = sml;
            }
        }
    });
    return lgs;
}

console.log(adjacentElementsProducts([3, 6, -2, -5, 7, 3]));