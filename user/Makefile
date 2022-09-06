
run_00:
	qemu-riscv64 ./target/riscv64gc-unknown-none-elf/release/00hello_world

run_01:
	qemu-riscv64 ./target/riscv64gc-unknown-none-elf/release/01store_fault

run_02:
	qemu-riscv64 ./target/riscv64gc-unknown-none-elf/release/02power

run_03:
	qemu-riscv64 ./target/riscv64gc-unknown-none-elf/release/03priv_inst

run_04:
	qemu-riscv64 ./target/riscv64gc-unknown-none-elf/release/04priv_csr

bin: build
	rust-objcopy --strip-all target/riscv64gc-unknown-none-elf/release/00hello_world -O binary target/riscv64gc-unknown-none-elf/release/00hello_world.bin
	rust-objcopy --strip-all target/riscv64gc-unknown-none-elf/release/01store_fault -O binary target/riscv64gc-unknown-none-elf/release/01store_fault.bin
	rust-objcopy --strip-all target/riscv64gc-unknown-none-elf/release/02power -O binary target/riscv64gc-unknown-none-elf/release/02power.bin
	rust-objcopy --strip-all target/riscv64gc-unknown-none-elf/release/03priv_inst -O binary target/riscv64gc-unknown-none-elf/release/03priv_inst.bin
	rust-objcopy --strip-all target/riscv64gc-unknown-none-elf/release/04priv_csr -O binary target/riscv64gc-unknown-none-elf/release/04priv_csr.bin

build:
	cargo build --release

clean:
	rm -rf ./target