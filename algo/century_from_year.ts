function centuryFromYear(year: number): number {
    let yr = year.toString().split("");
    let lst = yr[yr.length-1];
    if (parseInt(lst) > 0) {
        return parseInt(`${yr[0]}${yr[1]}`) + 1;
    }
    return parseInt(`${yr[0]}${yr[1]}`);
}

console.log(centuryFromYear(1905));
console.log(centuryFromYear(1700));