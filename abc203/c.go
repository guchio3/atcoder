package main

import (
	"bufio"
	"fmt"
	"os"
	"sort"
	"strconv"
	"strings"
)

func s2i(s string) int {
	v, ok := strconv.Atoi(s)
	if ok != nil {
		panic("Faild : " + s + " can't convert to int")
	}
	return v
}

type ab struct {
	a_i int
	b_i int
}

var sc = bufio.NewScanner(os.Stdin)

func main() {
	var n, k int
	if sc.Scan() {
		nk := strings.Split(sc.Text(), " ")
		n = s2i(nk[0])
		k = s2i(nk[1])
	}

	abs := make([]ab, 0)
	for i := int(0); i < n; i++ {
		var a_i, b_i int
		sc.Scan()
		a_i_b_i := strings.Split(sc.Text(), " ")
		a_i = s2i(a_i_b_i[0])
		b_i = s2i(a_i_b_i[1])
		abs = append(abs, ab{a_i: a_i, b_i: b_i})
	}

	sort.Slice(abs, func(i, j int) bool { return abs[i].a_i < abs[j].a_i })

	cur := int(0)
	for _, ab_i := range abs {
		diff := ab_i.a_i - cur
		if k-diff >= 0 {
			k -= diff
			cur += diff

			k += ab_i.b_i
		} else {
			break
		}
	}

	cur += k
	fmt.Printf("%d", cur)
}
