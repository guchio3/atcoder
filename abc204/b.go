package main

import (
	"bufio"
	"fmt"
	"os"
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

func main() {
	var n int
	fmt.Scan(&n)

	var sc = bufio.NewScanner(os.Stdin)
	sc.Scan()
	astrings := strings.Split(sc.Text(), " ")
	res := 0
	for _, as_i := range astrings {
		a_i := s2i(as_i)
		if a_i > 10 {
			res += a_i - 10
		}
	}
	fmt.Printf("%d", res)
}
