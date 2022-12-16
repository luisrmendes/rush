package utils

import (
	"fmt"
)

type PqElement struct {
	Priority int
	Name     string
}

type PriorityQueue []PqElement

func InsertElement(pq *PriorityQueue, newElement PqElement) {
	if len(*pq) == 0 {
		*pq = append(*pq, newElement)
		return
	}
	it := 0
	for _, elem := range *pq {
		if newElement.Priority >= elem.Priority {
			it++
			continue
		} else {
			break
		}
	}

	// Why the f i cannot index an pass by reference array??

	var pqCopy = *pq
	var pqCopy2 PriorityQueue

	var i = 0
	for ; i < it; i++ {
		pqCopy2 = append(pqCopy2, pqCopy[i])
	}
	
	pqCopy2 = append(pqCopy2, newElement)

	for ; i < len(pqCopy); i++ {
		pqCopy2 = append(pqCopy2, pqCopy[i])
	}

	*pq = pqCopy2
}

func Print(pq PriorityQueue) {
	fmt.Println("PriorityQueue print:")
	for _, elem := range pq {
		fmt.Printf("\t%d, %s\n", elem.Priority, elem.Name)
	}
}
