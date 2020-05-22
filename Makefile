MidProduct := play-args
FinalProduct := ${MidProduct}_
OutputDir := target/riscv64imac-unknown-none-elf/release
UserId := 1010
GroupId := 1010

ShouldInstall = `ckb-binary-patcher --version 2> /dev/null`

all: add_target build patch own

add_target:
	rustup target add riscv64imac-unknown-none-elf

build: add_target Cargo.toml src/main.rs
	cargo build --release

install_patcher:
    ifeq (${ShouldInstall},)
		cargo install --git https://github.com/xxuejie/ckb-binary-patcher.git
    endif

patch: install_patcher ${OutputDir}/${MidProduct}
	ckb-binary-patcher -i ${OutputDir}/${MidProduct} -o ${OutputDir}/${FinalProduct}

size:
	-@ls -l ${OutputDir}/${MidProduct} 2> /dev/null
	-@ls -l ${OutputDir}/${FinalProduct} 2> /dev/null

clean:
	cargo clean

own:
	chown -R ${UserId}:${GroupId} target Cargo.toml

.PHONY: clean size own
