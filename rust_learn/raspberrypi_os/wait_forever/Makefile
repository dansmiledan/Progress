define color_header
    @tput setaf 6 2> /dev/null || true
    @printf '\n%s\n' $(1)
    @tput sgr0 2> /dev/null || true
endef

define color_progress_prefix
    @tput setaf 2 2> /dev/null || true
    @tput bold 2 2> /dev/null || true
    @printf '%12s ' $(1)
    @tput sgr0 2> /dev/null || true
endef


ifeq ($(shell uname -s),Linux)
    DU_ARGUMENTS = --block-size=1024 --apparent-size
else ifeq ($(shell uname -s),Darwin)
    DU_ARGUMENTS = -k -A
endif

define disk_usage_KiB
    @printf '%s KiB\n' `du $(DU_ARGUMENTS) $(1) | cut -f1`
endef

##-------------------------------------------------
## user configuration values
##-------------------------------------------------
BSP ?= rpi3


##-------------------------------------------------
## BSP specific configuration values
##-------------------------------------------------
QEMU_MISSING_STRING = "this board is not yet support for qemu"

ifeq ($(BSP), rpi3)
    TARGET            = aarch64-unknown-none-softfloat
    KERNEL_BIN        = kernel8.img
    QEMU_BINARY       = qemu-system-aarch64
    QEMU_MACHINE_TYPE = raspi3b
    QEMU_RELEASE_ARGS = -serial stdio -display none
    OBJDUMP_BINARY    = aarch64-none-elf-objdump
    NM_BINARY         = aarch64-none-elf-nm
    READELF_BINARY    = aarch64-none-elf-readelf
    LD_SCRIPT_PATH    = $(shell pwd)/src/bsp/raspberrypi
    RUSTC_MISC_ARGS   = -C target-cpu=cortex-a53
else ifeq ($(BSP), rpi4)
    TARGET            = aarch64-unknown-none-softfloat
    KERNEL_BIN        = kernel8.img
    QEMU_BINARY       = qemu-system-aarch64
    QEMU_MACHINE_TYPE =
    QEMU_RELEASE_ARGS = -serial file:serial.out -display none
    OBJDUMP_BINARY    = aarch64-none-elf-objdump
    NM_BINARY         = aarch64-none-elf-nm
    READELF_BINARY    = aarch64-none-elf-readelf
    LD_SCRIPT_PATH    = $(shell pwd)/src/bsp/raspberrypi
    RUSTC_MISC_ARGS   = -C target-cpu=cortex-a72
endif

# Export for build.rs
export LD_SCRIPT_PATH

### Targets and prerequisites
KERNEL_MANIFEST = Cargo.toml
KERNEL_LINKER_SCRIPT = kernel.ld
LAST_BUILD_CONFIG = target/$(BSP).build_config

KERNEL_ELF = target/$(TARGET)/release/kernel
KERNEL_ELF_DEPS = $(filter-out %: , $(file < $(KERNEL_ELF).d)) $(KERNEL_MANIFEST) $(LAST_BUILD_CONFIG)


### Command building blocks
RUSTFLAGS = $(RUSTC_MISC_ARGS) \
	-C link-arg=--library-path=$(LD_SCRIPT_PATH) \
	-C link-arg=--script=$(KERNEL_LINKER_SCRIPT)

RUSTFLAGS_PEDANTIC = $(RUSTFLAGS)
# -D missing_docs
#-D warnings

FEATURES = --features bsp_$(BSP)
COMPILER_ARGS = --target=$(TARGET) \
	$(FEATURES) \
	--release

RUSTC_CMD = cargo rustc $(COMPILER_ARGS)
DOC_CMD = cargo doc $(COMPILER_ARGS)
CLIPPY_CMD = cargo clippy $(COMPILER_ARGS)
OBJCOPY_CMD = rust-objcopy \
	--strip-all \
	-O binary

EXEC_QEMU = $(QEMU_BINARY) -M $(QEMU_MACHINE_TYPE)
EXEC_TEST_DISPATCH = ruby tests/dispatch.rb

### Targets
.PHONY: all doc qemu clippy clean readelf objdump nm check test test_boot

all: $(KERNEL_BIN)

check:
	@echo $(KERNEL_ELF).d
	@echo $(file </Users/yintan/Documents/workspace/github/Progress/rust_learn/raspberrypi_os/wait_forever/build.rs)
	$(call color_header, $(file <$(KERNEL_ELF).d))


$(LAST_BUILD_CONFIG):
	@rm -f target/*.build_config
	@mkdir -p target
	@touch $(LAST_BUILD_CONFIG)

$(KERNEL_ELF): $(KERNEL_ELF_DEPS)
	$(call color_header, "Compiling kernel ELF - $(BSP)")
	$(call color_header, "Compiling kernel ELF - $(KERNEL_ELF_DEPS)")
	@RUSTFLAGS="$(RUSTFLAGS_PEDANTIC)" $(RUSTC_CMD)

$(KERNEL_BIN): $(KERNEL_ELF)
	$(call color_header, "Generating stripped binary")
	@$(OBJCOPY_CMD) $(KERNEL_ELF) $(KERNEL_BIN)
	$(call color_progress_prefix, "Name")
	@echo $(KERNEL_BIN)
	$(call color_progress_prefix, "Size")
	$(call disk_usage_KiB, $(KERNEL_BIN))

ifeq ($(QEMU_MACHINE_TYPE),)
qemu:
	$(call color_header, "$(QEMU_MISSING_STRING)")

else
qemu:
	$(call color_header, "Launching QEMU")
	@$(EXEC_QEMU) $(QEMU_RELEASE_ARGS) -kernel $(KERNEL_BIN)

test_boot: $(KERNEL_BIN)
	$(call color_header, "Boot test - $(BSP)")
	$(EXEC_TEST_DISPATCH) $(EXEC_QEMU) $(QEMU_RELEASE_ARGS) -kernel $(KERNEL_BIN)

test: test_boot

endif

##------------------------------------------------------------------------------
## Run clippy
##------------------------------------------------------------------------------
clippy:
	@RUSTFLAGS="$(RUSTFLAGS_PEDANTIC)" $(CLIPPY_CMD)

##------------------------------------------------------------------------------
## Clean
##------------------------------------------------------------------------------
clean:
	rm -rf target $(KERNEL_BIN)

##------------------------------------------------------------------------------
## Run readelf
##------------------------------------------------------------------------------
readelf: $(KERNEL_ELF)
	$(call color_header, "Launching readelf")
	@$(READELF_BINARY) --headers $(KERNEL_ELF)

##------------------------------------------------------------------------------
## Run objdump
##------------------------------------------------------------------------------
objdump: $(KERNEL_ELF)
	$(call color_header, "Launching objdump")
	@$(OBJDUMP_BINARY) --disassemble --demangle \
                --section .text   \
                $(KERNEL_ELF) | rustfilt

##------------------------------------------------------------------------------
## Run nm
##------------------------------------------------------------------------------
nm: $(KERNEL_ELF)
	$(call color_header, "Launching nm")
	@$(NM_BINARY) --demangle --print-size $(KERNEL_ELF) | sort | rustfilt
