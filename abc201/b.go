package main

import (
	"fmt"
	"sort"
)

type st struct {
	s_i string
	t_i int
}

func main() {
	var n int
	fmt.Scan(&n)

	sts := make([]st, n)
	for i := 0; i < n; i++ {
		var s_i string
		var t_i int
		fmt.Scan(&s_i, &t_i)

		sts[i] = st{s_i: s_i, t_i: t_i}
	}

	sort.Slice(sts, func(i, j int) bool { return sts[i].t_i > sts[j].t_i })
	fmt.Printf("%s", sts[1].s_i)
}
