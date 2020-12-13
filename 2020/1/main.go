package main

import (
    "fmt"
    "strconv"
    "os"
    "log"
    "bufio"
    "sort"
)

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

func solve(inp []int) (int, int) {
    p1, p2 := 0, len(inp) - 1
    fmt.Println(p1, p2)
    return inp[p1], inp[p2]
}

func main() {
    entries := parse("1/input.txt")
    sort.Ints(entries)
    fmt.Println(entries)
    e1, e2 := solve(entries)
    fmt.Printf("The final product is: %d\n", e1*e2)
}
