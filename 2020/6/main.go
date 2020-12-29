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

func calculateGroupCount(group string) int {
	people := splitPeople(group)
	var sets []mapset.Set
	for _, p := range people {
		characterSlice := make([]interface{}, len(p))
		for i := range p {
			characterSlice[i] = p[i]
		}
		characterSet := mapset.NewSetFromSlice(characterSlice)
		fmt.Println(characterSet)
		sets = append(sets, characterSet)
	}
	var finalSet mapset.Set
	for i := 0; i < len(sets)-1; i++ {
		if i == 0 {
			finalSet = sets[i].Intersect(sets[i+1])
		} else {
			finalSet = finalSet.Intersect(sets[i+1])
		}
	}
	fmt.Println(finalSet.Cardinality())
	return 0
}

func main() {
	raw := rawParse("6/input.txt")
	groups := splitGroups(raw)
	for _, g := range groups {
		calculateGroupCount(g)
	}
}
