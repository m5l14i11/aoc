from collections import Counter
import heapq

def solution(data, multiply=1):
    queue = [(0, (0,0))]
    heapq.heapify(queue)

    def get_neighbours(x, y):
        return [(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)]
    
    N, M = len(data), len(data[0])
    rows, cols = N * multiply, M * multiply
    done = (rows - 1, cols - 1)

    visited = set()
    counter = Counter()

    def get_risk(x, y):
        val = (data[x % N][y % M] + (x // N) + (y // M))
        return (val - 1) % 9 + 1
    
    while queue:
        risk, cords = heapq.heappop(queue)
        
        if cords in visited:
            continue

        visited.add(cords)

        counter[cords] = risk

        if cords in done:
            break

        for x, y in get_neighbours(*cords):
            if 0 <= x < rows and 0 <= y < cols:
                heapq.heappush(queue, (risk + get_risk(x, y), (x, y)))
    
    return counter[(rows - 1, cols - 1)]


with open('15.txt') as f:
    data = [[int(l) for l in line.strip()] for line in f.readlines()]
    ans1 = solution(data)
    print(ans1)
    ans2 = solution(data, 5)
    print(ans2)