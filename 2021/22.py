from collections import Counter

def solution_one(cubes, nrange): 
    ans1 = 0
    low, hight = nrange
    reactor = [[[0 for _ in range(0, hight + 1)] for _ in range(0, hight + 1)] for _ in range(0, hight + 1)]

    for (switch, coords) in cubes:
        x, y, z = coords[0], coords[1], coords[2]
        xmin, xmax = x[0] - low, x[1] - low
        ymin, ymax = y[0] - low, y[1] - low
        zmin, zmax = z[0] - low, z[1] - low

        if xmin < 0 or xmax > hight or ymin < 0 or ymax > hight or zmin < 0 or zmax > hight:
            continue

        for i in range(xmin, xmax + 1):
            for j in range(ymin, ymax + 1):
                for k in range(zmin, zmax + 1):
                    if switch:
                        reactor[i][j][k] = 1
                    else:
                        reactor[i][j][k] = 0
    
    for i in range(len(reactor)):
        for j in range(len(reactor[i])):
            ans1 += sum(reactor[i][j])

    return ans1

def solution_two(cubes):
    ans2 = 0
    return ans2


with open('22.txt') as f:
    raw_data = f.read().strip().split("\n")
    cubes = [
        (line[:3].strip() == 'on', [(int(coord.split('..')[0][2:]), int(coord.split('..')[1])) for coord in line[3:].strip().split(',')]) for line in raw_data
    ]
    ans1 = solution_one(cubes, (-50, 100))
    print(ans1)

    ans2 = solution_two(cubes)
    print(ans2)