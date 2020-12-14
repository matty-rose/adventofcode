package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	mapstructure "github.com/mitchellh/mapstructure"
	"strings"
	// "encoding/json"
)

type Passport struct {
    byr string
    iyr string
    eyr string
    hgt string
    hcl string
    ecl string
    pid string
    cid *string
}

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

func groupPassports(lines []string) []string {
    var tempString string
    var out []string
    for _, l := range lines {
        if len(l) == 0 {
            out = append(out, tempString)
            tempString = ""
        }
        tempString += " " + l
    }
    return out
}

func parsePassportString(p string) Passport {
    split := strings.Split(p, " ")
    var passport Passport
    passportMap := make(map[string]interface{})
    for _, s := range split {
        ssplit := strings.Split(s, ":")
        if len(ssplit) > 1 {
            passportMap[ssplit[0]] = ssplit[1]
        }
    }
    fmt.Println(passportMap)
    err := mapstructure.Decode(passportMap, &passport)
    if err != nil {
        log.Fatal(err)
    }
    fmt.Print(passport)
    return passport
}

func parsePassports(passports []string) []Passport {
    var out []Passport
    for _, p := range passports {
        out = append(out, parsePassportString(p))
    }
    return out
}

func main() {
    raw := rawParse("4/input.txt")
    passportStrings := groupPassports(raw)
    // parsePassports(passportStrings)
    passports := parsePassports(passportStrings)
    fmt.Println(passports)
}
