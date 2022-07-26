#!/bin/bash
EGO=boc-go/target/boc-go
ERS=boc-rs/target/release/boc-rs

events=1000
queue=256
etype=0
esize=64
verbose=

while getopts "e:q:t:s:" opt; do
  case ${opt} in
    e )
      events=$OPTARG
      ;;
    q )
      queue=$OPTARG
      ;;
    t )
      etype=$OPTARG
      ;;
    s )
      esize=$OPTARG
      ;;
  esac
done

echo "program,etype,worker,event,time,speed"
for worker in 100 1000 5000 10000 50000
do
    for etype in 0 1 2
    do
        ${EGO} -c -w ${worker} -e ${events} -q ${queue} -t ${etype} -s ${esize}
        echo
        ${ERS} -c -w ${worker} -e ${events} -q ${queue} -t ${etype} -s ${esize}
    done
done

