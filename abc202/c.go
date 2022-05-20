package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
)

var sc = bufio.NewScanner(os.Stdin)

func nextInt() int {
	sc.Scan()
	i, e := strconv.Atoi(sc.Text())
	if e != nil {
		panic(e)
	}
	return i
}

func main() {
	sc.Split(bufio.ScanWords)

	n := nextInt()
	a := make([]int, 0)
	b := make([]int, 0)
	c := make([]int, 0)
	for i := 0; i < n; i++ {
		a = append(a, nextInt())
	}
	for i := 0; i < n; i++ {
		b = append(b, nextInt())
	}
	for i := 0; i < n; i++ {
		c = append(c, nextInt())
	}

	b_map := make(map[int]int, 0)
	for i := 0; i < n; i++ {
		b_map[b[c[i]-1]] += 1
	}

	res := 0
	for i := 0; i < n; i++ {
		if v, ok := b_map[a[i]]; ok {
			res += v
		}
	}

	fmt.Printf("%d", res)
}
