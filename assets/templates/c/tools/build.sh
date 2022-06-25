#!/bin/bash

spliter () { echo =================================================================; }

if [ ! -d ./build/ ]; then mkdir ./build/; fi; spliter;

gcc -std=gnu17 -O3 -I ./include ./$1 -o ./build/$2; spliter;

if [ -f ./build/$2 ] && [ -x ./build/$2 ]; then
    ./build/$2;
fi; spliter;

rm -Rf ./build/*;
