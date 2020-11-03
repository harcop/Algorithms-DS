num = 5


def fact(n):
    if n == 0:
        return 1
    else:
        return n * fact(n-1)


def combination(m, n):
    a = fact(m)
    b = fact(m-n)
    c = a/(fact(n) * b)
    return int(c)


def pascal(n):
    for i in range(0, n):
        hash = (n-1)-i
        d = [hash*'#']
        for j in range(i+1):
            com = combination(i, j)
            d.append(com)
        print(d)


print(pascal(num))
