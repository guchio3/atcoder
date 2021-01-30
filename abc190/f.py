class Bit:
    def __init__(self, n):
        self.size = n
        self.tree = [0] * (n + 1)

    def sum(self, i):
        s = 0
        while i > 0:
            s += self.tree[i]
            i -= i & -i
        return s

    def add(self, i, x):
        while i <= self.size:
            self.tree[i] += x
            i += i & -i


n = int(input())
a = [int(a_i) + 1 for a_i in input().split()]

# a = reversed(a)

bit = Bit(n)
ans = 0

for i, a_i in enumerate(a):
    bit.add(a_i, 1)
    ans += i + 1 - sum(a)

print(ans)
