#!/bin/bash

input="./filelist.txt"
while IFS= read -r line
do 
    touch "$line"
done < "$input"

# while IFS='-' read -ra DATE line
# do 
#     for i in "${DATE[@]}"; do
#         echo $i
#     done
#     # touch "$line"
#     # echo -n $line | md5sum | awk '{print $1}'
# done < "$input"

#rm *.mkv