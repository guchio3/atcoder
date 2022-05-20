package main

import "fmt"

func main() {
	var s string
	fmt.Scan(&s)

	valid_map := make([]bool, len(s))
	valid_map[0] = true

	for i := 0; i < len(s); i++ {
		if !valid_map[i] {
			continue
		}
		if len(s)-i >= 5 {
			if s[i:i+5] == "dream" || s[i:i+5] == "erase" {
				if i+5 == len(s) {
					fmt.Println("YES")
					return
				} else {
					valid_map[i+5] = true
				}
			}
		}
		if len(s)-i >= 6 {
			if s[i:i+6] == "eraser" {
				if i+6 == len(s) {
					fmt.Println("YES")
					return
				} else {
					valid_map[i+6] = true
				}
			}
		}
		if len(s)-i >= 7 {
			if s[i:i+7] == "dreamer" {
				if i+7 == len(s) {
					fmt.Println("YES")
					return
				} else {
					valid_map[i+7] = true
				}
			}
		}
	}
	fmt.Println("NO")
}
