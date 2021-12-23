#
# Makefile
#

SHELL=/bin/bash
.DEFAULT_GOAL=help

.PHONY: _check-day-defined
_check-day-defined:
	@if [[ -z "${DAY}" ]]; then printf "\nThe DAY environment variable is not defined\n\n"; exit 1; fi

.PHONY: day
day: _check-day-defined
	cp src/days/template.rs src/days/day${DAY}.rs
	echo "mod day${DAY};" >> src/days/mod.rs
