def first_solution(data):
    horiz = 0
    depth = 0
    for (move, pos) in data:
        pos = int(pos)
        if move == 'forward':
            horiz += pos
        if move == 'down':
            depth += pos
        if move == 'up':
            depth -= pos

    return horiz * depth

def second_solution(data):
    horiz = 0
    depth = 0
    aim = 0

    for (move, pos) in data:
        pos = int(pos)
        if move == 'forward':
            horiz += pos
            depth += aim * pos
        if move == 'down':
            aim += pos
        if move == 'up':
            aim -= pos

    return horiz * depth


with open('2.txt') as f:
    data = [tuple(x.strip().split(' ')) for x in f.readlines()]
    ans1 = first_solution(data)
    print(ans1)
    ans2 = second_solution(data)
    print(ans2)
    
    

