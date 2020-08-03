test = int(input("enter case "))
for i in range(test):
    n = int(input('len of int '))
    int_n = input('enter int ').split(" ")
    k = 1
    for j in int_n:
        k *= int(j)
    if k % 2 == 0:
        print("NO")
    else:
        print("YES")