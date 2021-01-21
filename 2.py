from collections import Counter

with open('2.txt', encoding = 'utf-8') as f:
    lines = f.readlines()
    ans1 = 0
    ans2 = 0

    for line in lines:
        [n, symbol, password] = line.strip().split(' ')
        
        n_range = n.split('-')
        symbol = symbol.replace(':', '')

        lower = int(n_range[0])
        upper = int(n_range[1])

        c = Counter(password)
        count = c[symbol]

        if count >= lower and count <= upper:
            ans1 += 1

        first = password[lower - 1] == symbol
        second = password[upper - 1] == symbol

        if (first and not second) or (second and not first):
            ans2 += 1
        
    print(ans1)
    print(ans2)
        