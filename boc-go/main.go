package main

import (
	"bytes"
	"flag"
	"fmt"
	"sync/atomic"
	"time"
)

// SN is the golbal atomic sn of tasks
var SN = int64(0)

func worker(queue chan []byte, done chan int64, doneTarget int64) {
	for {
		event := <-queue
		sn := atomic.AddInt64(&SN, 1)
		if sn == doneTarget {
			done <- sn
		}
		if len(event) == 4 && bytes.Compare(event, []byte("quit")) == 0 {
			break
		}
	}
}

func dispatch(events, eventSize int, address []chan []byte) {
    for i := 0; i < events; i++ {
        go func() {
            for _, addr := range address {
                event := make([]byte, eventSize)
                addr <- event
            }
        }()
    }
}

func main() {

	eventSize := flag.Int("s", 16, "the size of event")
	workers := flag.Int("w", 100, "the count of workers")
	events := flag.Int("e", 100, "the count of events")
	queueSize := flag.Int("q", 16, "the size of worker queue")
	csvFormat := flag.Bool("c", false, "output as csv format")
	verbose := flag.Bool("v", false, "more output")
	flag.Parse()

	done := make(chan int64, 1)
	// address is the send port of all workers
	address := make([]chan []byte, *workers)
	target := int64(*workers * *events)

	// startup workers
	for i := 0; i < *workers; i++ {
		queue := make(chan []byte, *queueSize)
		address[i] = queue
		go worker(queue, done, target)
	}

	// t1 is the time of startup
	t1 := time.Now()

	// spawn the dispatch
	go dispatch(*events, *eventSize, address)

	// wait all task done
	sn := <-done
	if *verbose {
		// verfiy done is real done
		fmt.Println("done target", sn)
	}

	t2 := time.Now()
	ts := t2.Sub(t1).Seconds()
	speed := float64(target) / ts

	if *csvFormat {
		fmt.Printf("go,%d,%d,%.3f,%.3f", *workers, *events, ts, speed)
	} else {
		fmt.Printf("workers   : %d\n", *workers)
		fmt.Printf("events    : %d\n", *events)
		fmt.Printf("time used : %.3fS\n", ts)
		fmt.Printf("Speed     : %.3f/S\n", speed)
	}
}
