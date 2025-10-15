# Run `cargo bench`.
#	1) name of the program
#   2) bench name
.PHONY: bench

# Remove the first word (the target) to get positional arguments
ARG1 := $(word 2, $(MAKECMDGOALS)) # program name
ARG2 := $(word 3, $(MAKECMDGOALS)) # bench name

bench:
	@# Temporarily move .cargo to avoid using local config during benchmarks
	@mv .cargo .cargo-temp
	cargo +nightly bench --bench $(ARG1) -- $(ARG2)
	@mv .cargo-temp .cargo

# Build the program.
.PHONY: build
build:
	cargo build-bpf --manifest-path programs/pinocchio/Cargo.toml
	@# RUSTFLAGS="-C embed-bitcode=yes -C lto=fat" cargo build-sbf --manifest-path programs/sdk/Cargo.toml --tools-version v1.51

# Run `cargo clean`.
.PHONY: clean
clean:
	cargo clean

# Run `cargo clippy`.
.PHONY: clippy
clippy:
	cargo clippy

# Run `cargo fmt`.
.PHONY: format
format:
	cargo fmt

# Catch-all rule to prevent errors when target is not
# match (this happens with command-line arguments)
%:
	@:
