new:
	touch data/day$(day).txt
	touch src/challenges/day$(day).rs
	echo "pub mod day$(day)" >> src/challenges/mod.rs
