const getDuplicatesFromArray = function(array){
    let duplicates = [];

    array.forEach(function(item) {
        if(array.filter(val => val === item).length > 1 && duplicates.filter(val => val === item).length === 0){
            duplicates.push(item);
        }
    });

    return duplicates;
};


const returnDuplicate = (array) => {
  const duplicates = [];
  const seens = {};
  array.forEach((item) => seens[item] ? duplicates.push(item) : seens[item] = item)
  return duplicates
}

console.log(getDuplicatesFromArray([1,1,3,5,6,3,7]))
console.log(returnDuplicate([1,1,3,5,6,3,7]))

