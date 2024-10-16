#!/usr/bin/env bash

# generate coverage report
cargo tarpaulin --force-clean --out Html --engine llvm --output-dir "$(pwd)/target/coverage-report"

# display link to coverage report
echo "Report: file://$(pwd)/target/coverage-report/tarpaulin-report.html"
