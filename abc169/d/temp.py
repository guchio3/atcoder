def prime_factorize(n):
    a = []
    while n % 2 == 0:
        a.append(2)
        n //= 2
    f = 3
    while f * f <= n:
        if n % f == 0:
            a.append(f)
            n //= f
        else:
            f += 2
    if n != 1:
        a.append(n)
    return a


a = int(input().split()[0])
prime_res = prime_factorize(a)

prime_dict = {}
for prime_i in prime_res:
    if prime_i not in prime_dict:
        prime_dict[prime_i] = 1
    else:
        prime_dict[prime_i] += 1

res = 0
for key in prime_dict:
    base = 1
    while True:
        if prime_dict[key] < base:
            break
        else:
            prime_dict[key] -= base
            base += 1
            res += 1

print(res)
