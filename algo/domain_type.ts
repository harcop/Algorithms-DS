function domainType(d: string[]): string[] {
    let _dt = [];
    d.forEach(ele => {
        let _t = ele.split('.');
        switch(_t[_t.length-1]) {
            case('org'):
                _dt.push('organisation');
                break;
            case('com'):
                _dt.push('communication');
                break;
            case('net'):
                _dt.push('network');
                break;
            case('info'):
                _dt.push('information');
                break;
        }
    });
    return _dt;
}

console.log(domainType(["en.wiki.org", "codefights.com", "happy.net", "code.info"]));