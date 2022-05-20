package main

import (
	"fmt"
)

func main() {
	var s string
	fmt.Scan(&s)

	ss := []rune(s)

	for i, s_i := range ss {
		switch s_i {
		case '6':
			ss[i] = '9'
		case '9':
			ss[i] = '6'
		}
	}
	for i := 0; i < int(len(ss)/2); i++ {
		ss[i], ss[len(ss)-i-1] = ss[len(ss)-i-1], ss[i]
	}

	fmt.Printf("%s", string(ss))
}
