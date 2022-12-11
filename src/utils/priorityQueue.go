package utils

import (
	"fmt"
)

type PqElement struct {
	Priority int
	Name string
}

type PriorityQueue []PqElement

func InsertElement(pq *PriorityQueue, newElement PqElement) {
	if len(*pq) == 0 {
		*pq = append(*pq, newElement)
		return
	}
	it := 0
	for _, elem := range *pq {
		if newElement.Priority > elem.Priority {
			it++
			continue
		} else {
			break
		}
	}

	// Why the f i cannot index an pass by reference array??

	var pqCopy = *pq

	var firstSlice = pqCopy[:it]
	firstSlice = append(firstSlice, newElement)
	var secondSlice = pqCopy[it:]
	*pq = append(firstSlice, secondSlice[:]...)
}

func Print(pq PriorityQueue) {
	fmt.Println("PriorityQueue print:")
	for _, elem := range pq {
		fmt.Printf("\t%d, %s\n", elem.Priority, elem.Name)
	}
}