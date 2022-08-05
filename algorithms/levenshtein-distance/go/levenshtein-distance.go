package main

import "fmt"

func min(args ...int) int {
	minimum := args[0]

	for _, i := range args {
		if minimum > i {
			minimum = i
		}
	}

	return minimum
}

func levenshteinDistancePrivate(one, two string, len1, len2 int) int {
	if len1 == 0 {
		return len2
	}

	if len2 == 0 {
		return len1
	}

	cost := 0

	if one[len1-1] != two[len2-1] {
		cost = 1
	}

	return min(levenshteinDistancePrivate(one, two, len1-1, len2)+1, levenshteinDistancePrivate(one, two, len1, len2-1)+1, levenshteinDistancePrivate(one, two, len1-1, len2-1)+cost)
}

func levenshteinDistance(one, two string) int {
	return levenshteinDistancePrivate(one, two, len(one), len(two))
}

func main() {
	one := "Hello"
	two := "Jello"

	distance := levenshteinDistance(one, two)

	fmt.Println(distance)
}
