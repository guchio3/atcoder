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
	var sc = bufio.NewScanner(os.Stdin)
	sc.Scan()
	n := s2i(sc.Text())

	sc.Scan()
	stra := strings.Split(sc.Text(), " ")
	used := make([]bool, n)

	flg := false
	for i := 0; i < n; i++ {
		if used[s2i(stra[i])-1] {
			flg = true
		}
		used[s2i(stra[i])-1] = true
	}

	if flg {
		fmt.Printf("No")
	} else {
		fmt.Printf("Yes")
	}
}
