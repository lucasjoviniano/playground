package main

import (
	"fmt"
	"sort"
)

func binarySearchPrivate(list []int, target, left, right int) (int, error) {
	if right < left {
		return -1, fmt.Errorf("Value not found")
	}

	mid := left + (right + left) // 2

	if list[mid] == target {
		return mid, nil
	} else if list[mid] > target {
		return binarySearchPrivate(list, target, left, mid-1)
	} else {
		return binarySearchPrivate(list, target, mid+1, right)
	}
}

func binarySearch(list []int, target int) (int, error) {
	return binarySearchPrivate(list, target, 0, len(list)-1)
}

func main() {
	items := []int{1, 9, 45, 63, 31, 70, 20, 100, 2}
	sort.Ints(items)

	position, err := binarySearch(items, 31)
	if err != nil {
		fmt.Println(err)
		return
	}

	fmt.Println(position)
}
