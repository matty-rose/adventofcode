package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
)

type slope struct {
	right int
	down  int
}

func parse(location string) []string {
	f, err := os.Open(location)
	if err != nil {
		log.Fatal(err)
	}
	defer f.Close()
	scanner := bufio.NewScanner(f)
	var out []string
	for scanner.Scan() {
		out = append(out, scanner.Text())
	}
	return out
}

func solve(inp []string, right, down int) int {
	x, y := 0, 0
	var tree_count int = 0
	for y < len(inp)-1 {
		x = (x + right) % len(inp[0])
		y += down
		if string(inp[y][x]) == "#" {
			tree_count += 1
		}
	}
	return tree_count
}

func p2(inp []string) int {
	var prod int = 1
	var slopes = [][]int{[]int{1, 1}, []int{3, 1}, []int{5, 1}, []int{7, 1}, []int{1, 2}}
	for _, s := range slopes {
		count := solve(inp, s[0], s[1])
		prod *= count
	}
	return prod
}

func main() {
	lines := parse("3/input.txt")
	fmt.Println(lines)
	p1_solution := solve(lines, 3, 1)
	fmt.Printf("%d trees encountered in problem 1\n", p1_solution)
	p2_solution := p2(lines)
	fmt.Printf("%d trees encountered in problem 2\n", p2_solution)
}
