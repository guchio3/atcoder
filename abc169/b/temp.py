input()
a = input().rstrip().split()
a = [int(a_i) for a_i in a]

res = 1
if 0 in a:
    print(0)
else:
    for a_i in a:
        a_i = int(a_i)
        res = res * a_i
        if res > 1000000000000000000:
            print(-1)
            break
    else:
        print(res)
