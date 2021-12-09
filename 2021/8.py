def solution_one(data):
    return sum([
        len(chars) in (2, 3, 4, 7)
        for line in data
        for chars in line[1].split() 
    ])

with open('8.txt') as f:
    data = [tuple(line.strip().split(' | ')) for line in f.readlines()]
    ans1 = solution_one(data)
    print(ans1)