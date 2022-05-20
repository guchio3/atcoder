package main

import (
	"fmt"
	"sort"
)

func main() {
	var n int
	fmt.Scan(&n)

	a := make([]int, n)
	for i := 0; i < n; i++ {
		var tmp int
		fmt.Scan(&tmp)
		a = append(a, tmp)
	}
	sort.Slice(a, func(i, j int) bool { return a[i] > a[j] })

	alice_sum := 0
	bob_sum := 0
	for i, v := range a {
		if i%2 == 0 {
			alice_sum += v
		} else {
			bob_sum += v
		}
	}
	fmt.Println(alice_sum - bob_sum)
}
