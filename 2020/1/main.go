package main

import (
    "fmt"
    "os"
    "log"
    "bufio"
)

func parse(location string) {
    f, err := os.Open(location)
    if err != nil {
        log.Fatal(err)
    }
    defer f.Close()
    scanner := bufio.NewScanner(f)
    for scanner.Scan() {
        fmt.Println(scanner.Text())
    }
}

func sort() {
}

func solve() {
}

func main() {
    raw_input = parse("1/input.txt")
    sorted_list = sort(raw_input)
    e1, e2 = solve(sorted_list)
    fmt.Printf("The final product is: %d\n", e1*e2)
}
