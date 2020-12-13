package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strconv"
	"strings"
)

type entry struct {
	min      int
	max      int
	letter   string
	password string
}

func parse(location string) []entry {
	f, err := os.Open(location)
	if err != nil {
		log.Fatal(err)
	}
	defer f.Close()
	scanner := bufio.NewScanner(f)
	var out []entry
	for scanner.Scan() {
		line := scanner.Text()
		line = strings.Replace(line, ":", "", 1)
		split_line := strings.Split(line, " ")
		minmax, char, pw := split_line[0], split_line[1], split_line[2]
		split_minmax := strings.Split(minmax, "-")
		min, max := split_minmax[0], split_minmax[1]
		mini, _ := strconv.Atoi(min)
		maxi, _ := strconv.Atoi(max)
		out = append(out, entry{min: mini, max: maxi, letter: char, password: pw})
	}
	return out
}

func check_valid_password_p1(entry entry) bool {
	char_count := strings.Count(entry.password, entry.letter)
	if (char_count >= entry.min) && (char_count <= entry.max) {
		return true
	} else {
		return false
	}
}

func check_valid_password_p2(entry entry) bool {
	pos1 := string(entry.password[entry.min-1])
	pos2 := string(entry.password[entry.max-1])
	if ((pos1 == entry.letter) && (pos2 != entry.letter)) || ((pos1 != entry.letter) && (pos2 == entry.letter)) {
		return true
	} else {
		return false
	}
}

func solve_problem(problem int, entries []entry, fn func(entry) bool) {
	var success_count int = 0
	for _, e := range entries {
		valid := fn(e)
		if valid {
			success_count += 1
		}
	}
	fmt.Printf("Count of valid passwords for problem %d: %d\n", problem, success_count)
}

func main() {
	entries := parse("2/input.txt")
	solve_problem(1, entries, check_valid_password_p1)
	solve_problem(2, entries, check_valid_password_p2)
}
