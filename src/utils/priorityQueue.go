package utils

import (
	"fmt"
)

type PqElement struct {
	Priority int
	Name     string
}

type PriorityQueue []PqElement

func RemoveElementFromName(pq *PriorityQueue, elemName string) {
	var i = 0
	for _, v := range *pq {
		if v.Name == elemName {
			break
		}
		i++
	}

	newSlice := append((*pq)[:i], (*pq)[i+1:]...)
	copy(*pq, newSlice)
	(*pq) = (*pq)[:len(newSlice)]
}

func RemoveElement(pq *PriorityQueue, elem PqElement) {
	var i = 0
	for _, v := range *pq {
		if v.Name == elem.Name {
			break
		}
		i++
	}

	newSlice := append((*pq)[:i], (*pq)[i+1:]...)
	copy(*pq, newSlice)
	(*pq) = (*pq)[:len(newSlice)]
}

func NewPqElement(priority int, name string) *PqElement {
	var newElement PqElement
	newElement.Priority = priority
	newElement.Name = name
	return &newElement
}

func NewPriorityQueue(pqElements ...*PqElement) PriorityQueue {
	var newPriorityQueue PriorityQueue
	for _, v := range pqElements {
		newPriorityQueue = append(newPriorityQueue, *v)
	}
	return newPriorityQueue
}

func InsertElement(pq *PriorityQueue, newElement PqElement) {
	if len(*pq) == 0 {
		*pq = append(*pq, newElement)
		return
	}

	// Why the f i cannot index an pass by reference array??

	var pqCopy = *pq
	var pqCopy2 PriorityQueue

	var i = 0
	for ; i < len(pqCopy); i++ {
		if pqCopy[i].Priority <= newElement.Priority {
			pqCopy2 = append(pqCopy2, pqCopy[i])
		} else {
			break
		}
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
