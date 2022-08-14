#!/bin/bash

input="./filelist.txt"
while IFS = read -r line
do 
    touch "$line"
done < "$input"