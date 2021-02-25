function findEmailDomain(a: string): string {
    let _a = a.split("@");
    return _a[_a.length - 1];
}

console.log(findEmailDomain('prettyandsimple@example.com'));
console.log(findEmailDomain('<>[]:,;@\"!#$%&*+-/=?^_{}| ~.a\"@example.org'));
