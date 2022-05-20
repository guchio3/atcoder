package main

import (
	"fmt"
	"sort"
)

func main() {
	a := make([]int, 3)
	var a1, a2, a3 int
	fmt.Scan(&a1, &a2, &a3)

	a[0] = a1
	a[1] = a2
	a[2] = a3

	sort.IntSlice(a).Sort()

	if a[1]-a[0] == a[2]-a[1] {
		fmt.Printf("Yes")
	} else {
		fmt.Printf("No")
	}
}
