#!/bin/bash
ret=0
for file in ./tests_bin/*; do
    echo $file $@
    ./$file $@
    if [ $? -ne 0 ];
    then
	    ret=-1
        break
    fi
    echo $ret
done

if [ "$ret" -ne 0 ]; 
then 
    echo "test failed"; 
else 
    echo "ALL TEST PASSED"
fi
