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
		splitLine := strings.Split(line, " ")
		minmax, char, pw := splitLine[0], splitLine[1], splitLine[2]
		splitMinMax := strings.Split(minmax, "-")
		min, max := splitMinMax[0], splitMinMax[1]
		mini, _ := strconv.Atoi(min)
		maxi, _ := strconv.Atoi(max)
		out = append(out, entry{min: mini, max: maxi, letter: char, password: pw})
	}
	return out
}

func checkValidPasswordP1(entry entry) bool {
	charCount := strings.Count(entry.password, entry.letter)
	if (charCount >= entry.min) && (charCount <= entry.max) {
		return true
	}
	return false
}

func checkValidPasswordP2(entry entry) bool {
	pos1 := string(entry.password[entry.min-1])
	pos2 := string(entry.password[entry.max-1])
	if ((pos1 == entry.letter) && (pos2 != entry.letter)) || ((pos1 != entry.letter) && (pos2 == entry.letter)) {
		return true
	}
	return false
}

func solveProblem(problem int, entries []entry, fn func(entry) bool) {
	var successCount int = 0
	for _, e := range entries {
		valid := fn(e)
		if valid {
			successCount++
		}
	}
	fmt.Printf("Count of valid passwords for problem %d: %d\n", problem, successCount)
}

func main() {
	entries := parse("2/input.txt")
	solveProblem(1, entries, checkValidPasswordP1)
	solveProblem(2, entries, checkValidPasswordP2)
}
