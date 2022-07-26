#!/bin/bash
EGO=boc-go/target/boc-go
ERS=boc-rs/target/release/boc-rs

echo "program,worker,event,time,speed"
for worker in 100 1000 2000 5000 10000 50000
do
    for event in 1000
    do
        ${EGO} -c -w ${worker} -e ${event} -q 256
        echo
        ${ERS} -c -w ${worker} -e ${event} -q 256
    done
done

