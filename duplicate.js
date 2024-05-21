const getDuplicatesFromArray = function(array){
    let duplicates = [];

    array.forEach(function(item) {
        if(array.filter(val => val === item).length > 1 && duplicates.filter(val => val === item).length === 0){
            duplicates.push(item);
        }
    });

    return duplicates;
};

const start = performance.now();
console.log(getDuplicatesFromArray([1,1,3,5,6,3,7]))
const end = performance.now();
const duration = end - start;
console.log(`${duration} milliseconds.`);


const returnDuplicate = (array) => {
  const duplicates = [];
  const seens = {};
  array.forEach((item) => seens[item] ? duplicates.push(item) : seens[item] = true)
  return duplicates
}

const start2 = performance.now();
console.log(returnDuplicate([1,1,3,5,6,3,7]))
const end2 = performance.now();
const duration2 = end2 - start2;
console.log(`${duration2} milliseconds.`);
