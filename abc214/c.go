package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

var reader = NewReader()

func intMin(x, y int) int {
	if x < y {
		return x
	} else {
		return y
	}
}

func main() {
	n := reader.Int()
	s := reader.Ints(n)
	t := reader.Ints(n)

	min_t_i := 1_000_000_001
	min_t_idx := -1
	for i := 0; i < n; i++ {
		if t[i] < min_t_i {
			min_t_i = t[i]
			min_t_idx = i
		}
	}

	reses := make([]int, n)
	reses[min_t_idx] = min_t_i
	for i := (min_t_idx + 1) % n; i != min_t_idx; i = (i + 1) % n {
		var j int
		if i == 0 {
			j = n - 1
		} else {
			j = i - 1
		}
		reses[i] = intMin(reses[j]+s[j], t[i])
	}

	for _, res_i := range reses {
		fmt.Printf("%d\n", res_i)
	}
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
