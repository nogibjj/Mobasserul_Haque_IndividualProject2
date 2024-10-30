rust-version:
	@echo "Rust command-line utility versions:"
	rustc --version 			#rust compiler
	cargo --version 			#rust package manager
	rustfmt --version			#rust code formatter
	rustup --version			#rust toolchain manager
	clippy-driver --version		#rust linter

format:
	cargo fmt --quiet

install:
	# Install if needed
	#@echo "Updating rust toolchain"
	#rustup update stable
	#rustup default stable 

lint:
	cargo clippy --quiet

test:
	cargo test --quiet

run:
	cargo run

release:
	cargo build --release

all: format lint test run

extract: 
	cargo run extract

load:
	cargo run load
# Read 
read:
	cargo run -- query "SELECT * FROM AirlineSafety LIMIT 10;"
# Delete
delete:
	cargo run -- query "DELETE FROM AirlineSafety WHERE id = 1;"  
# Insert
insert:
	cargo run -- query "INSERT INTO AirlineSafety (airline, avail_seat_km_per_week, incidents_85_99, fatal_accidents_85_99, fatalities_85_99, incidents_00_14, fatal_accidents_00_14, fatalities_00_14) VALUES ('Air Canada', 1865253802, 2, 0, 0, 2, 0, 0);"
# Update
update:
	cargo run -- query "UPDATE AirlineSafety SET airline = 'Air Canada', avail_seat_km_per_week = 2000000000, incidents_85_99 = 3, fatal_accidents_85_99 = 1, fatalities_85_99 = 5, incidents_00_14 = 2, fatal_accidents_00_14 = 0, fatalities_00_14 = 0 WHERE id = 1;"