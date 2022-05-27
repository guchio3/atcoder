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
		if v == "" {
			continue
		}
		v, _ := strconv.Atoi(v)
		ans = append(ans, v)
	}
	return ans
}

func s2i(s string) int {
	v, ok := strconv.Atoi(s)
	if ok != nil {
		panic("Faild : " + s + " can't convert to int")
	}
	return v
}

func main() {
	var n, q int
	fmt.Scan(&n, &q)

	a := reader.Ints(n)

	for i := 0; i < q; i++ {
		k_i := reader.Int()
		bef_less_cnt := 0
		less_cnt := sort.Search(len(a), func(j int) bool { return a[j] >= k_i })
		if less_cnt < n && a[less_cnt] == k_i {
			less_cnt += 1
		}
		for less_cnt != bef_less_cnt {
			bef_less_cnt = less_cnt
			less_cnt = sort.Search(len(a), func(j int) bool { return a[j] >= k_i+less_cnt })
			if less_cnt < n && a[less_cnt] == k_i+less_cnt {
				less_cnt += 1
			}
		}

		fmt.Printf("%d\n", k_i+less_cnt)
	}
}
