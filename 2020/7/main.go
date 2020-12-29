package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
)

func rawParse(location string) []string {
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

func main() {
	raw := rawParse("7/input.txt")
	fmt.Println(raw)
}
