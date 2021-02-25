function addBorder(picture: string[]): string[] {
    var wall = '*'.repeat(picture[0].length);
    picture.unshift(wall);
    picture.push(wall); 
    picture.forEach((ele,index) => {
        picture[index] = `*${ele}*`;
    })
    return picture;
}

console.log(addBorder(["abc", "ded"]));