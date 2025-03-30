#!/bin/bash

# run tests with a single thread as we test the stdout
cargo test -- --test-threads=1
