from itertools import permutations


def factorization(n):
    arr = []
    temp = n
    for i in range(2, int(-(-n**0.5 // 1)) + 1):
        if temp % i == 0:
            cnt = 0
            while temp % i == 0:
                cnt += 1
                temp //= i
            arr.append([i, cnt])

    if temp != 1:
        arr.append([temp, 1])

    if arr == []:
        arr.append([n, 1])

    return arr


s, p = input().split()
s = int(s)
p = int(p)

p_facts = factorization(p)


res_facts = []
for i in range(2**len(p_facts)):
    facts = [1]
    for j in range(len(p_facts)):
        if (i >> j) & 1:
            tmp_facts = []
            for fact in facts:
                for k in range(1, p_facts[j][1] + 1):
                    tmp_fact = fact * (p_facts[j][0] ** k)
                    if tmp_fact > s:
                        break
                    tmp_facts.append(tmp_fact)
            facts = tmp_facts
    res_facts += facts

for fact in res_facts:
    if (fact + p / fact) == s:
        print("Yes")
        exit(0)
print("No")
