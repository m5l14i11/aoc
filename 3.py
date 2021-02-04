from functools import reduce

forest = []

with open('3.txt', encoding = 'utf-8') as f:
    forest = [x.strip() for x in f.readlines()]

def numOfTrees(arr, dx=0, dy=0):
    x = 0
    y = 0

    result = 0
    width = len(arr[0])

    while y < len(arr):
        if arr[y][x] == '#':
            result += 1

        y += dy
        x = (x + dx) % width

    return result

ans1 = numOfTrees(forest, 3, 1)
print(ans1)

ans2 = reduce(lambda x, y: x * y, [numOfTrees(forest, dx, dy) for (dx, dy) in [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]])
print(ans2)