#!/bin/bash
EGO=boc-go/target/boc-go
ERS=boc-rs/target/release/boc-rs

worker=1000
events=1000
queue=256
etype=0
esize=64
verbose=

while getopts "w:e:q:t:s:v" opt; do
  case ${opt} in
    w )
      worker=$OPTARG
      ;;
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
    v )
      verbose=-v
      ;;
  esac
done

echo "program,worker,event,time,speed"
if [ -n "${verbose}" ]; then
  echo ${EGO} -c -w ${worker} -e ${events} -q ${queue} -t ${etype} -s ${esize} ${verbose}
fi
${EGO} -c -w ${worker} -e ${events} -q ${queue} -t ${etype} -s ${esize} ${verbose}
echo
if [ -n "${verbose}" ]; then
  echo ${ERS} -c -w ${worker} -e ${events} -q ${queue} -t ${etype} -s ${esize} ${verbose}
fi
${ERS} -c -w ${worker} -e ${events} -q ${queue} -t ${etype} -s ${esize} ${verbose}

