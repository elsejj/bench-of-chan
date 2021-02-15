#!/bin/bash
EGO=boc-go/target/boc-go
ERS=boc-rs/target/release/boc-rs

echo "program,worker,event,time,speed"
for worker in 100 1000 10000 100000
do
    for event in 100 500 1000 2000
    do
        ${EGO} -c -w ${worker} -e ${event}
        echo
        ${ERS} -c -w ${worker} -e ${event}
    done
done

