package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

// func rev(s []rune) []rune {
func rev(s []rune) {
	for i := 0; i < len(s)/2; i++ {
		tmp := s[i]
		s[i] = s[len(s)-i-1]
		s[len(s)-i-1] = tmp
	}
	// return s
}

var reader = NewReader()

func main() {
	s := reader.String()
	ss := []rune(s)
	MOD := 1_000_000_007

	rev(ss)

	cnts := make([]int, 8)
	for _, ss_i := range ss {
		switch ss_i {
		case 'c':
			cnts[7] = (cnts[7] + cnts[6]) % MOD
		case 'h':
			cnts[6] = (cnts[6] + cnts[5]) % MOD
		case 'o':
			cnts[5] = (cnts[5] + cnts[4]) % MOD
		case 'k':
			cnts[4] = (cnts[4] + cnts[3]) % MOD
		case 'u':
			cnts[3] = (cnts[3] + cnts[2]) % MOD
		case 'd':
			cnts[2] = (cnts[2] + cnts[1]) % MOD
		case 'a':
			cnts[1] = (cnts[1] + cnts[0]) % MOD
		case 'i':
			cnts[0] = (cnts[0] + 1) % MOD
		}
	}
	fmt.Printf("%d", cnts[7])
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
