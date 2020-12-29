package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"reflect"
	"regexp"
	"strconv"
	"strings"
)

type passport struct {
	Byr *int64
	Iyr *int64
	Eyr *int64
	Hgt *string
	Hcl *string
	Ecl *string
	Pid *string
	Cid *string
}

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

func contains(list []string, val string) bool {
	for _, item := range list {
		if item == val {
			return true
		}
	}
	return false
}

func parsePassportString(p string) passport {
	split := strings.Split(p, " ")
	var pp passport

	for _, s := range split {
		if s == "" {
			continue
		}
		ssplit := strings.Split(s, ":")
		if contains([]string{"byr", "iyr", "eyr"}, ssplit[0]) {
			intval, err := strconv.ParseInt(ssplit[1], 10, 64)
			if err != nil {
				log.Fatal(err)
			}
			reflect.ValueOf(&pp).Elem().FieldByName(strings.Title(ssplit[0])).Set(reflect.ValueOf(&intval))
		} else {
			reflect.ValueOf(&pp).Elem().FieldByName(strings.Title(ssplit[0])).Set(reflect.ValueOf(&ssplit[1]))
		}
	}
	return pp
}

func parsePassports(passports []string) []passport {
	var out []passport
	for _, p := range passports {
		out = append(out, parsePassportString(p))
	}
	return out
}

func solveP1(passports []passport) int {
	validCount := 0
	for _, pp := range passports {
		nilCount := 0
		if pp.Byr == nil {
			nilCount++
		}
		if pp.Eyr == nil {
			nilCount++
		}
		if pp.Iyr == nil {
			nilCount++
		}
		if pp.Hgt == nil {
			nilCount++
		}
		if pp.Hcl == nil {
			nilCount++
		}
		if pp.Ecl == nil {
			nilCount++
		}
		if pp.Pid == nil {
			nilCount++
		}
		if nilCount == 0 {
			validCount++
		}
	}
	return validCount
}

func solveP2(passports []passport) int {
	validCount := 0
	for _, pp := range passports {
		nilCount := 0
		valid := true
		if pp.Byr != nil {
			if *pp.Byr > 2002 || *pp.Byr < 1920 {
				valid = false
			}
		} else {
			nilCount++
		}
		if pp.Iyr != nil {
			if *pp.Iyr > 2020 || *pp.Iyr < 2010 {
				valid = false
			}
		} else {
			nilCount++
		}
		if pp.Eyr != nil {
			if *pp.Eyr > 2030 || *pp.Eyr < 2020 {
				valid = false
			}
		} else {
			nilCount++
		}
		if pp.Hgt != nil {
			hgt := *pp.Hgt
			measurement := hgt[len(hgt)-2:]
			val, _ := strconv.Atoi(hgt[:len(hgt)-2])
			switch measurement {
			case "cm":
				if val < 150 || val > 193 {
					valid = false
				}
			case "in":
				if val < 59 || val > 76 {
					valid = false
				}
			default:
				valid = false
			}
		} else {
			nilCount++
		}
		if pp.Hcl != nil {
			match, _ := regexp.MatchString("#[a-fA-F0-9]{6}", *pp.Hcl)
			if !match {
				valid = false
			}
		} else {
			nilCount++
		}
		if pp.Ecl != nil {
			match, _ := regexp.MatchString("(amb)|(blu)|(brn)|(gry)|(grn)|(hzl)|(oth)", *pp.Ecl)
			if !match {
				valid = false
			}
		} else {
			nilCount++
		}
		if pp.Pid != nil {
			match, _ := regexp.MatchString("[0-9]{9}", *pp.Pid)
			if !match {
				valid = false
			}
		} else {
			nilCount++
		}
		if nilCount == 0 && valid {
			validCount++
		}
	}
	return validCount
}

func main() {
	raw := rawParse("4/input.txt")
	passportStrings := groupPassports(raw)
	passports := parsePassports(passportStrings)
	res1 := solveP1(passports)
	fmt.Println("Problem 1 result: ", res1)
	res2 := solveP2(passports)
	fmt.Println("Problem 2 result: ", res2)
}
