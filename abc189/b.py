(n, x) = input().split()
n = int(n)
x = int(x)

cum = 0
for i in range(n):
    (v, p) = input().split()
    v = int(v)
    p = int(p)
    cum += v * p / 100
    if cum > x:
        print(i + 1)
        exit(0)
print(-1)
