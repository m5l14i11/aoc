from collections import deque
from itertools import islice

def first_solution(measurements):
    ans = 0
    
    for i in range(1, len(measurements)):
        if int(measurements[i]) > int(measurements[i - 1]):
            ans += 1

    return ans

def sliding_window(iterable, size):
    iterable = iter(iterable)
    window = deque(islice(iterable, size - 1), maxlen=size)

    for item in iterable:
        window.append(item)
        yield tuple(window)

def second_solution(measurements):
    prev_sum = 0
    ans = 0

    for t in sliding_window(measurements, 3):
        ss = int(t[0]) + int(t[1]) + int(t[2])

        if ss > prev_sum:
            ans += 1
        
        prev_sum = ss

    return ans - 1

with open('1.txt', encoding = 'utf-8') as f:
    measurements = [x.strip() for x in f.readlines()]
    ans1 = first_solution(measurements)
    print(ans1)
    ans2 = second_solution(measurements)
    print(ans2)