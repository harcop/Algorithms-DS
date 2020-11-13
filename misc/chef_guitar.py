no_test = int(input('please enter number of test cases '))
# print(abs(-12))
for i in range(0, no_test):
    string__total_plug = int(input('please enter the string total plugged '))
    string_plug = input('please enter the string plugged ').split(" ")
    gap_skip = 0
    for j in range(0, len(string_plug)-1):
        # print(j, string_plug[j])
        skip = abs(int(string_plug[j+1]) - int(string_plug[j]))
        if skip is not 0:
            skip -= 1
        gap_skip += skip
    print(gap_skip)
