package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strings"
)

func getRequiredFields() []string {
	return []string{"byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"}
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
	var tempString string = ""
	var out []string
	for i, l := range lines {
		tempString += " " + l
		if len(l) == 0 || i == len(lines)-1 {
			out = append(out, tempString)
			tempString = ""
		}
	}
	return out
}

func checkValidPassport(passport string) bool {
	requiredFields := getRequiredFields()
	var valid bool = true
	for _, field := range requiredFields {
		if !strings.Contains(passport, field) {
			valid = false
		}
	}
	return valid
}

func solveP1(passports []string) int {
	var count int = 0
	for _, pp := range passports {
		if checkValidPassport(pp) {
			count++
		}
	}
	return count
}

func solveP2(passports []string) int {
}

// func parsePassportString(p string) passport {
// 	split := strings.Split(p, " ")
// 	var pp passport
// 	passportMap := make(map[string]interface{})
// 	for _, s := range split {
// 		ssplit := strings.Split(s, ":")
// 		if len(ssplit) > 1 {
// 			passportMap[ssplit[0]] = ssplit[1]
// 		}
// 	}
// 	fmt.Println(passportMap)
// 	err := mapstructure.Decode(passportMap, &pp)
// 	if err != nil {
// 		log.Fatal(err)
// 	}
// 	fmt.Println(pp)
// 	return pp
// }

// func parsePassports(passports []string) []passport {
// 	var out []passport
// 	for _, p := range passports {
// 		out = append(out, parsePassportString(p))
// 	}
// 	return out
// }

func main() {
	raw := rawParse("4/input.txt")
	passportStrings := groupPassports(raw)
	res := solveP1(passportStrings)
	fmt.Println(res)
	// parsePassports(passportStrings)
	// passports := parsePassports(passportStrings)
	// fmt.Println(passports)
}
