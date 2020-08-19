function htmlEndTagByStartTag(st: string): string {
    let _t = st.split(" ")[0].split("<")[1].split('>')[0];
    console.log(`</${_t}>`);
    return `</${_t}>`;
}

console.log(htmlEndTagByStartTag("<button type='button' disabled>"));
console.log(htmlEndTagByStartTag('<i>'))