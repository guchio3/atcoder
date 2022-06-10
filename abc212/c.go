package main

import (
	"bufio"
	"fmt"
	"os"
	"sort"
	"strconv"
	"strings"
)

var reader = NewReader()

func intAbs(x int) int {
	if x >= 0 {
		return x
	} else {
		return -x
	}
}

func intMin(x, y int) int {
	if x < y {
		return x
	} else {
		return y
	}
}

func main() {
	n, m := reader.IntPair()
	a := reader.Ints(n)
	b := reader.Ints(m)

	sort.Ints(a)
	sort.Ints(b)

	res := intAbs(a[0] - b[0])
	i := 0
	j := 0

	// for i < n && j < m {
	// 	res = intMin(res, intAbs(a[i] - b[j]))
	// 	if a[i] > b[j] {
	// 		j += 1
	// 	} else {
	// 		i += 1
	// 	}
	// }
	for ; i < n; i++ {
		for j < m-1 && intAbs(a[i]-b[j]) > intAbs(a[i]-b[j+1]) {
			j += 1
		}

		res = intMin(res, intAbs(a[i]-b[j]))
	}

	fmt.Printf("%d", res)
}

// Read
type Reader struct {
	r *bufio.Reader
}

func NewReader() Reader {
	return Reader{bufio.NewReader(os.Stdin)}
}

func (r Reader) String() string {
	line, _ := reader.r.ReadString('\n')
	return strings.TrimRight(line, "\r\n")
}

func (r Reader) Int() int {
	line := r.String()
	v, _ := strconv.Atoi(line)
	return v
}

func (r Reader) IntPair() (int, int) {
	v := strings.Split(r.String(), " ")
	v0, _ := strconv.Atoi(v[0])
	v1, _ := strconv.Atoi(v[1])
	return v0, v1
}

func (r Reader) IntTriple() (int, int, int) {
	v := strings.Split(r.String(), " ")
	v0, _ := strconv.Atoi(v[0])
	v1, _ := strconv.Atoi(v[1])
	v2, _ := strconv.Atoi(v[2])
	return v0, v1, v2
}

func (r Reader) IntQuad() (int, int, int, int) {
	v := strings.Split(r.String(), " ")
	v0, _ := strconv.Atoi(v[0])
	v1, _ := strconv.Atoi(v[1])
	v2, _ := strconv.Atoi(v[2])
	v3, _ := strconv.Atoi(v[3])
	return v0, v1, v2, v3
}

func (r Reader) Ints(n int) []int {
	ans := make([]int, 0, n)
	for _, v := range strings.Split(r.String(), " ") {
		v, _ := strconv.Atoi(v)
		ans = append(ans, v)
	}
	return ans
}

func (r Reader) StringPair() (string, string) {
	v := strings.Split(r.String(), " ")
	return v[0], v[1]
}

func (r Reader) Strings(n int) []string {
	res := make([]string, 0, n)
	for _, v := range strings.Split(r.String(), " ") {
		res = append(res, v)
	}
	return res
}
