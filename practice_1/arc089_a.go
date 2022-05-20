package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

func intabs(v int) int {
	if v < 0 {
		v = -v
	}
	return v
}

func main() {
	scanner := bufio.NewScanner(os.Stdin)
	scanner.Scan()
	// var n int
	// fmt.Scan(&n)
	n, _ := strconv.Atoi(scanner.Text())

	ti, xi, yi := 0, 0, 0
	for i := 0; i < n; i++ {
		scanner.Scan()
		s := strings.Split(scanner.Text(), " ")

		// var ti1, xi1, yi1 int
		// fmt.Scan(&ti1, &xi1, &yi1)
		ti1, _ := strconv.Atoi(s[0])
		xi1, _ := strconv.Atoi(s[1])
		yi1, _ := strconv.Atoi(s[2])

		distance := intabs(xi1-xi) + intabs(yi1-yi)
		if (ti1-ti) < distance || ((ti1-ti)-distance)%2 != 0 {
			fmt.Println("No")
			return
		}

		ti, xi, yi = ti1, xi1, yi1
	}
	fmt.Println("Yes")
}
