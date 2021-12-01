data = []

with open('4.txt', encoding = 'utf-8') as f:
    data = [x.strip() for x in f.readlines()]

print(data)