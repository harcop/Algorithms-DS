test = int(input())

for j in range(test):
    n = int(input())

    digit = {
        1: 2,
        2: 5,
        3: 5,
        4: 4,
        5: 5,
        6: 6,
        7: 3,
        8: 7,
        9: 6,
        0: 6
    }

    _ds = [2, 3]

    _fd = 0

    lst = str(n)

    for i in lst:
        _fd = _fd + digit[int(i)]


    def spl(_fd):
        if _fd % 2:
            _n = (_fd/2) - 1
            _l = 7
            while _n > 1:

                _l = (_l * 10) + 1
                _n -= 1
            return _l
        _l = 0
        while _fd > 1:
            _l = (_l * 10) + 1
            _fd -= 2
        return _l


    print(spl(_fd))
