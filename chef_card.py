no_test = int(input('please enter number of test '))
final = []
for i in range(0, no_test):
    no_round = int(input('please enter number of rounds '))
    round_point = [0, 0]
    for j in range(0, no_round):
        round_this = [0, 0]
        round_card = input('please enter round card outcome ')
        card = round_card.split(" ")
        n = 0
        for k in card:
            p = 0
            for m in list(k):
                p += int(m)
                round_this[n] = p
            n += 1
        if round_this[0] > round_this[1]:
            round_point[0] += 1
        elif round_this[1] > round_this[0]:
            round_point[1] += 1
        else:
            round_point[0] += 1
            round_point[1] += 1
    if round_point[0] > round_point[1]:
        result = [0, round_point[0]]
        # print(result)
        final.append(result)
    elif round_point[1] > round_point[0]:
        result = [1, round_point[1]]
        final.append(result)
        # print(1, round_point[1])
    else:
        result = [2, round_point[1]]
        final.append(result)
        # print(2, round_point[1])
for i in final:
    print(*i)
