package main

import (
	"flag"
	"fmt"
	"log"
	"os"
	"runtime/pprof"
	"strings"
	"sync/atomic"
	"time"
)

// SN is the golbal atomic sn of tasks
var SN = int64(0)

type Event interface {
	IsExit() bool
	Clone() Event
}

type IntEvent struct {
	Val int64
}

type StrEvent struct {
	Val string
}

func NewIntEvent(seed int64) Event {
	return IntEvent{seed}
}

func NewStrEvent(seed int64) Event {
	return StrEvent{strings.Repeat("A", int(seed))}
}

func (e IntEvent) IsExit() bool {
	return e.Val == -1
}

func (e IntEvent) Clone() Event {
	return IntEvent{e.Val}
}

func (e StrEvent) IsExit() bool {
	return e.Val == "exit"
}

func (e StrEvent) Clone() Event {
	return StrEvent{strings.Clone(e.Val)}
}

func worker(queue chan Event, done chan int64, doneTarget int64) {
	for i := int64(0); i < doneTarget; i++ {
		event := <-queue
		atomic.AddInt64(&SN, 1)
		if event.IsExit() {
			break
		}
	}
	close(queue)
	done <- 1
}

func dispatchTo(event Event, events int64, addr chan Event) {
	for i := int64(0); i < events; i++ {
		copiedEvent := event.Clone()
		addr <- copiedEvent
	}
}

func dispatch(events, eventSize int64, address []chan Event, eventType int) {
	var event Event
	switch eventType {
	case 0:
		event = NewIntEvent(eventSize)
	case 1:
		event = NewStrEvent(eventSize)
	default:
		panic(fmt.Sprintf("invalid event type %d", eventType))
	}
	for i := 0; i < len(address); i++ {
		go dispatchTo(event, events, address[i])
	}
}

func main() {

	workers := flag.Int64("w", 100, "the count of workers")
	events := flag.Int64("e", 100, "the count of events")
	queueSize := flag.Int64("q", 16, "the size of worker queue")
	csvFormat := flag.Bool("c", false, "output as csv format")
	verbose := flag.Bool("v", false, "more output")
	eventType := flag.Int("t", 0, "event type, 0 for int, 1 for str")
	eventSize := flag.Int64("s", 16, "the size of event, no used when event type is int(0)")
	cpuprofile := flag.String("cpuprofile", "", "write cpu profile to `file`")
	flag.Parse()

	if *cpuprofile != "" {
		f, err := os.Create(*cpuprofile)
		if err != nil {
			log.Fatal("could not create CPU profile: ", err)
		}
		defer f.Close() // error handling omitted for example
		if err := pprof.StartCPUProfile(f); err != nil {
			log.Fatal("could not start CPU profile: ", err)
		}
		defer pprof.StopCPUProfile()
	}

	done := make(chan int64, *workers)
	// address is the send port of all workers
	address := make([]chan Event, *workers)
	totalEvents := *workers * *events

	// startup workers
	for i := int64(0); i < *workers; i++ {
		queue := make(chan Event, *queueSize)
		address[i] = queue
		go worker(queue, done, *events)
	}

	// t1 is the time of startup
	t1 := time.Now()

	// spawn the dispatch
	go dispatch(*events, *eventSize, address, *eventType)

	// wait all task done
	for i := int64(0); i < *workers; i++ {
		<-done
	}
	if *verbose {
		// verfiy done is real done
		fmt.Println("total events", totalEvents, "done events", atomic.LoadInt64(&SN))
	}

	t2 := time.Now()
	ts := t2.Sub(t1).Seconds()
	speed := float64(totalEvents) / ts

	if *csvFormat {
		fmt.Printf("go,%d,%d,%.3f,%.3f", *workers, *events, ts, speed)
	} else {
		fmt.Printf("workers   : %d\n", *workers)
		fmt.Printf("events    : %d\n", *events)
		fmt.Printf("time used : %.3fS\n", ts)
		fmt.Printf("Speed     : %.3f/S\n", speed)
	}
}
