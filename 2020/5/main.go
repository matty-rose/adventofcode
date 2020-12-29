package main

import (
	"bufio"
	"fmt"
	"log"
	"math"
	"os"
	"sort"
)

const rows int = 127
const seats int = 7

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

func getSeatID(s string) int {
	rowMin, rowMax, finalRow := 0, rows, 0
	seatMin, seatMax, finalSeat := 0, seats, 0
	for i, char := range s {
		if i <= 6 {
			rowDelta := int(math.Pow(2, float64(6-i)))
			switch string(char) {
			case "F":
				if i == 6 {
					finalRow = rowMin
				} else {
					rowMax -= rowDelta
				}
			case "B":
				if i == 6 {
					finalRow = rowMax
				} else {
					rowMin += rowDelta
				}
			}
		} else {
			seatDelta := int(math.Pow(2, float64(3-(i-6))))
			switch string(char) {
			case "L":
				if (i - 6) == 3 {
					finalSeat = seatMin
				} else {
					seatMax -= seatDelta
				}
			case "R":
				if (i - 6) == 3 {
					finalSeat = seatMax
				} else {
					seatMin += seatDelta
				}
			}
		}
	}
	seatID := finalRow*8 + finalSeat
	return seatID
}

func solveP1(seatStrings []string) (int, []int) {
	currentMax := 0
	var ids []int
	for _, str := range seatStrings {
		id := getSeatID(str)
		if id >= currentMax {
			currentMax = id
		}
		ids = append(ids, id)
	}
	return currentMax, ids
}

func solveP2(seatIDs []int) int {
	sort.Ints(seatIDs)
	var mine int
	for i := range seatIDs {
		if i == len(seatIDs)-1 {
			break
		}
		diff := seatIDs[i+1] - seatIDs[i]
		if diff == 2 {
			mine = seatIDs[i] + 1
		}
	}
	return mine
}

func main() {
	raw := rawParse("5/input.txt")
	maxID, Ids := solveP1(raw)
	fmt.Println("Max seat ID in problem 1 is: ", maxID)
	myID := solveP2(Ids)
	fmt.Println("My seat ID in problem 2 is: ", myID)
}
