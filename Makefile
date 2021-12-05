new:
	touch data/day$(day).txt
	echo "use crate::lib::read_lines_from;\n\nstruct Input { }\n\nfn read_input() -> Input {\n  let mut lines = read_lines_from(\"./data/day$(day).txt\").unwrap();\n  Input {}\n}\n\npub fn part_one() {\n \n}\n\npub fn part_two() {\n \n}" > src/challenges/day$(day).rs
	echo "pub mod day$(day);" >> src/challenges/mod.rs
