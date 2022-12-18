package utils

import (
	"testing"
)

func TestRemovePQ4(t *testing.T) {
	t.Log("priorityQueue Remove test 4")

	var testPq PriorityQueue = NewPriorityQueue(NewPQElement(1, "temples of syrinx"))

	err := RemovePQElement(&testPq, "By Tor")

	if err == nil {
		t.Errorf("Method should throw \"Element with name %s not found!\"", "By Tor")
		t.Fail()
	}
}

func TestRemovePQ3(t *testing.T) {
	t.Log("priorityQueue Remove test 3")

	var testPq PriorityQueue

	err := RemovePQElement(&testPq, "By Tor")

	if err == nil {
		t.Errorf("Method should throw \"Priority Queue is empty!\"")
		t.Fail()
	}
}

func TestRemovePQ2(t *testing.T) {
	t.Log("priorityQueue Remove test 2")

	var testPq PriorityQueue = NewPriorityQueue(NewPQElement(1, "temples of syrinx"), NewPQElement(2, "snow dog"),
		NewPQElement(3, "By Tor"), NewPQElement(5, "2112"), NewPQElement(10, "Caravan"))

	var correctPq = NewPriorityQueue(NewPQElement(2, "snow dog"),
		NewPQElement(5, "2112"))

	RemovePQElement(&testPq, "By Tor")
	RemovePQElement(&testPq, "Caravan")
	RemovePQElement(&testPq, "temples of syrinx")

	if len(testPq) != len(correctPq) {
		t.Errorf("priorityQueue size is %d, expected %d", len(correctPq), len(testPq))
		t.Fail()
	}

	for i := 0; i < len(correctPq); i++ {
		if correctPq[i] != testPq[i] {
			t.Errorf("priorityQueue element is (%d, %s), expected (%d, %s)",
				correctPq[i].Priority, correctPq[i].Name,
				testPq[i].Priority, testPq[i].Name)
			t.Fail()
		}
	}
}

func TestRemovePQ1(t *testing.T) {
	t.Log("priorityQueue Remove test 1")

	var testPq PriorityQueue = NewPriorityQueue(NewPQElement(1, "temples of syrinx"), NewPQElement(2, "snow dog"),
		NewPQElement(3, "By Tor"), NewPQElement(5, "2112"), NewPQElement(10, "Caravan"))

	var correctPq = NewPriorityQueue(NewPQElement(1, "temples of syrinx"), NewPQElement(2, "snow dog"),
		NewPQElement(3, "By Tor"), NewPQElement(10, "Caravan"))

	RemovePQElement(&testPq, "2112")

	if len(testPq) != len(correctPq) {
		t.Errorf("priorityQueue size is %d, expected %d", len(correctPq), len(testPq))
		t.Fail()
	}

	for i := 0; i < len(correctPq); i++ {
		if correctPq[i] != testPq[i] {
			t.Errorf("priorityQueue element is (%d, %s), expected (%d, %s)",
				correctPq[i].Priority, correctPq[i].Name,
				testPq[i].Priority, testPq[i].Name)
			t.Fail()
		}
	}
}

func TestPriorityQueueInsert3(t *testing.T) {
	t.Log("priorityQueue Insert test 3")

	var testPq PriorityQueue
	var correctPq = NewPriorityQueue(NewPQElement(1, "temples of syrinx"), NewPQElement(2, "snow dog"),
		NewPQElement(3, "By Tor"), NewPQElement(5, "2112"), NewPQElement(10, "Caravan"))

	var elem1 = *NewPQElement(3, "By Tor")
	var elem2 = *NewPQElement(10, "Caravan")
	var elem3 = *NewPQElement(1, "temples of syrinx")
	var elem4 = *NewPQElement(2, "snow dog")
	var elem5 = *NewPQElement(5, "2112")

	InsertPQElement(&testPq, elem1)
	InsertPQElement(&testPq, elem2)
	InsertPQElement(&testPq, elem3)
	InsertPQElement(&testPq, elem4)
	InsertPQElement(&testPq, elem5)

	if len(testPq) != len(correctPq) {
		t.Errorf("priorityQueue size is %d, expected %d", len(correctPq), len(testPq))
		t.Fail()
	}

	for i := 0; i < len(correctPq); i++ {
		if correctPq[i] != testPq[i] {
			t.Errorf("priorityQueue element is (%d, %s), expected (%d, %s)",
				correctPq[i].Priority, correctPq[i].Name,
				testPq[i].Priority, testPq[i].Name)
			t.Fail()
		}
	}
}

func TestPriorityQueueInsert2(t *testing.T) {
	t.Log("priorityQueue Insert test 2")

	var testPq PriorityQueue
	var correctPq = NewPriorityQueue(NewPQElement(1, "temples of syrinx"),
		NewPQElement(2, "snow dog"), NewPQElement(3, "By Tor"))

	var elem1 = *NewPQElement(1, "temples of syrinx")
	var elem2 = *NewPQElement(2, "snow dog")
	var elem3 = *NewPQElement(3, "By Tor")

	InsertPQElement(&testPq, elem1)
	InsertPQElement(&testPq, elem2)
	InsertPQElement(&testPq, elem3)

	if len(testPq) != len(correctPq) {
		t.Errorf("priorityQueue size is %d, expected %d", len(correctPq), len(testPq))
		t.Fail()
	}

	for i := 0; i < len(correctPq); i++ {
		if correctPq[i] != testPq[i] {
			t.Errorf("priorityQueue element is (%d, %s), expected (%d, %s)",
				correctPq[i].Priority, correctPq[i].Name,
				testPq[i].Priority, testPq[i].Name)
			t.Fail()
		}
	}
}

func TestPriorityQueueInsert1(t *testing.T) {
	t.Log("priorityQueue Insert test 1")

	var testPq PriorityQueue
	var correctPq = NewPriorityQueue(NewPQElement(1, "temples of syrinx"))

	var elem = *NewPQElement(1, "temples of syrinx")
	InsertPQElement(&testPq, elem)

	if len(testPq) != len(correctPq) {
		t.Errorf("priorityQueue size is %d, expected %d", len(testPq), len(correctPq))
		t.Fail()
	}

	for i := 0; i < len(correctPq); i++ {
		if correctPq[i] != testPq[i] {
			t.Errorf("priorityQueue element is (%d, %s), expected (%d, %s)",
				correctPq[i].Priority, correctPq[i].Name,
				testPq[i].Priority, testPq[i].Name)
			t.Fail()
		}
	}
}

func TestPriorityQueueConstructor(t *testing.T) {
	t.Log("PriorityQueue Constructor test")

	var testPq = NewPriorityQueue(NewPQElement(1, "temples of syrinx"))
	var newPq [1]PqElement
	newPq[0] = *NewPQElement(1, "temples of syrinx")

	for _, v := range testPq {
		if v != newPq[0] {
			t.Log("Error: Priority Queue elements are not the same!")
			t.Fail()
		}
	}
}

func TestPqElementConstructor(t *testing.T) {
	t.Log("PqElement Constructor test")

	var testElem = NewPQElement(1, "temples of syrinx")

	var newElem PqElement
	newElem.Priority = 1
	newElem.Name = "temples of syrinx"

	if testElem.Priority != newElem.Priority {
		t.Log("Error: Priorities are not the same!")
		t.Fail()
	}
	if testElem.Name != newElem.Name {
		t.Log("Error: Names are not the same!")
		t.Fail()
	}
}
