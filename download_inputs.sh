#!/bin/bash

COOKIES_FILE=adventofcode.com_cookies.txt
NUMBER_OF_DAYS=12
YEAR=2025

if ! test -d input;
then
  # create input dir
  mkdir input
fi

for i_without_leading in $(seq 1 $NUMBER_OF_DAYS);
do
  i_with_leading=$(printf "%02d" $i_without_leading)

  if [[ "$YEAR-12-10#$i_with_leading" -lt $(date +%Y-%_m-%_d) ]];
  then
      # this date is yet to come
      break
  fi

  if ! test -f "input/$i_with_leading.txt"; then
    # download input file
    wget --load-cookies=$COOKIES_FILE https://adventofcode.com/2025/day/$i_without_leading/input -O input/$i_with_leading.txt
  fi

done
