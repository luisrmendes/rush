package utils

import (
	"testing"
)

func TestPriorityQueueInsert3(t *testing.T) {
	t.Log("priorityQueue Insert test 3")

	var testPq PriorityQueue
	var correctPq = NewPriorityQueue(NewPqElement(1, "temples of syrinx"), NewPqElement(2, "snow dog"),
		NewPqElement(3, "By Tor"), NewPqElement(5, "2112"), NewPqElement(10, "Caravan"))

	var elem1 = *NewPqElement(3, "By Tor")
	var elem2 = *NewPqElement(10, "Caravan")
	var elem3 = *NewPqElement(1, "temples of syrinx")
	var elem4 = *NewPqElement(2, "snow dog")
	var elem5 = *NewPqElement(5, "2112")

	InsertElement(&testPq, elem1)
	InsertElement(&testPq, elem2)
	InsertElement(&testPq, elem3)
	InsertElement(&testPq, elem4)
	InsertElement(&testPq, elem5)

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
	var correctPq = NewPriorityQueue(NewPqElement(1, "temples of syrinx"),
		NewPqElement(2, "snow dog"), NewPqElement(3, "By Tor"))

	var elem1 = *NewPqElement(1, "temples of syrinx")
	var elem2 = *NewPqElement(2, "snow dog")
	var elem3 = *NewPqElement(3, "By Tor")

	InsertElement(&testPq, elem1)
	InsertElement(&testPq, elem2)
	InsertElement(&testPq, elem3)

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
	var correctPq = NewPriorityQueue(NewPqElement(1, "temples of syrinx"))

	var elem = *NewPqElement(1, "temples of syrinx")
	InsertElement(&testPq, elem)

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

	var testPq = NewPriorityQueue(NewPqElement(1, "temples of syrinx"))
	var newPq [1]PqElement
	newPq[0] = *NewPqElement(1, "temples of syrinx")

	for _, v := range testPq {
		if v != newPq[0] {
			t.Log("Error: Priority Queue elements are not the same!")
			t.Fail()
		}
	}
}

func TestPqElementConstructor(t *testing.T) {
	t.Log("PqElement Constructor test")

	var testElem = NewPqElement(1, "temples of syrinx")

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
