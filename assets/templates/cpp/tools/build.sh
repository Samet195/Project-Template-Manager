#!/bin/bash

spliter () { echo =================================================================; }

if [ ! -d ./build/ ]; then mkdir ./build/; fi; spliter;

cmake -S . -B ./build;
cmake --build ./build; spliter;

if [ -f ./build/main ] && [ -x ./build/main ]; then
    ./build/main;
fi; spliter;

rm -Rf ./build/*;
