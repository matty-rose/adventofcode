package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"sort"
	"strconv"
)

const total = 2020

func parse(location string) []int {
	f, err := os.Open(location)
	if err != nil {
		log.Fatal(err)
	}
	defer f.Close()
	scanner := bufio.NewScanner(f)
	var out []int
	for scanner.Scan() {
		i, err := strconv.Atoi(scanner.Text())
		if err != nil {
			log.Fatal(err)
		}
		out = append(out, i)
	}
	return out
}

func solvep1(inp []int) (int, int) {
	p1, p2 := 0, len(inp)-1
	var currentSum int
	for currentSum != total {
		currentSum = inp[p1] + inp[p2]
		if currentSum < total {
			p1++
		} else if currentSum > total {
			p2--
		}
	}
	return inp[p1], inp[p2]
}

func solvep2(inp []int) (int, int, int) {
	p1, p2, p3 := 0, 1, len(inp)-1
	var currentSum int
	flag := true
	for currentSum != total {
		currentSum = inp[p1] + inp[p2] + inp[p3]
		if currentSum < total {
			if flag {
				p2++
			} else {
				p1++
			}
			flag = !flag
		} else if currentSum > total {
			p3--
		}
	}
	return inp[p1], inp[p2], inp[p3]
}

func main() {
	entries := parse("1/input.txt")
	sort.Ints(entries)
	e1, e2 := solvep1(entries)
	fmt.Printf("The final product for part 1 is: %d\n", e1*e2)
	f1, f2, f3 := solvep2(entries)
	fmt.Printf("The final product for part 2 is: %d\n", f1*f2*f3)
}
