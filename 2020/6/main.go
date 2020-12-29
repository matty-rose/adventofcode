package main

import (
	"fmt"
	mapset "github.com/deckarep/golang-set"
	"io/ioutil"
	"log"
	"strings"
)

func rawParse(location string) string {
	b, err := ioutil.ReadFile(location)
	if err != nil {
		log.Fatal(err)
	}
	return string(b)
}

func splitGroups(raw string) []string {
	groups := strings.Split(raw, "\n\n")
	return groups
}

func splitPeople(group string) []string {
	people := strings.Split(group, "\n")
	return people
}

func getSets(group string) []mapset.Set {
	people := splitPeople(group)
	var sets []mapset.Set
	for _, p := range people {
		characterSlice := make([]interface{}, len(p))
		for i := range p {
			characterSlice[i] = p[i]
		}
		characterSet := mapset.NewSetFromSlice(characterSlice)
		if characterSet.Cardinality() != 0 {
			sets = append(sets, characterSet)
		}
	}
	return sets
}

func calculateUnionCount(group string) int {
	sets := getSets(group)
	finalSet := mapset.NewSet()
	if len(sets) == 1 {
		return sets[0].Cardinality()
	}
	for i := 0; i < len(sets)-1; i++ {
		if i == 0 {
			finalSet = sets[i].Union(sets[i+1])
		} else {
			finalSet = finalSet.Union(sets[i+1])
		}
	}
	return finalSet.Cardinality()
}

func calculateIntersectCount(group string) int {
	sets := getSets(group)
	finalSet := mapset.NewSet()
	if len(sets) == 1 {
		return sets[0].Cardinality()
	}
	for i := 0; i < len(sets)-1; i++ {
		if i == 0 {
			finalSet = sets[i].Intersect(sets[i+1])
		} else {
			finalSet = finalSet.Intersect(sets[i+1])
		}
	}
	return finalSet.Cardinality()
}

func solveP1(groups []string) int {
	var totalSum int
	for _, g := range groups {
		totalSum += calculateUnionCount(g)
	}
	return totalSum
}

func solveP2(groups []string) int {
	var totalSum int
	for _, g := range groups {
		totalSum += calculateIntersectCount(g)
	}
	return totalSum
}

func main() {
	raw := rawParse("6/input.txt")
	groups := splitGroups(raw)
	totalSumP1 := solveP1(groups)
	fmt.Println("Total sum for problem 1: ", totalSumP1)
	totalSumP2 := solveP2(groups)
	fmt.Println("Total sum for problem 1: ", totalSumP2)
}
