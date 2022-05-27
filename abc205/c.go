package main

import (
	"fmt"
)

func intAbs(a int) int {
	if a < 0 {
		a *= -1
	}
	return a
}

func main() {
	var a, b, c int
	fmt.Scan(&a, &b, &c)

	if a >= 0 && b >= 0 {
		if a > b {
			fmt.Printf(">")
		} else if a < b {
			fmt.Printf("<")
		} else {
			fmt.Printf("=")
		}
	} else if a <= 0 && b <= 0 {
		if c%2 == 0 {
			if a > b {
				fmt.Printf("<")
			} else if a < b {
				fmt.Printf(">")
			} else {
				fmt.Printf("=")
			}
		} else {
			if a > b {
				fmt.Printf(">")
			} else if a < b {
				fmt.Printf("<")
			} else {
				fmt.Printf("=")
			}
		}
	} else {
		if c%2 == 0 {
			if intAbs(a) > intAbs(b) {
				fmt.Printf(">")
			} else if intAbs(a) < intAbs(b) {
				fmt.Printf("<")
			} else {
				fmt.Printf("=")
			}
		} else {
			if a > 0 {
				fmt.Printf(">")
			} else {
				fmt.Printf("<")
			}
		}
	}
}
