RUSTC  = rustc
TARGET = thumbv7m-linux-elf
CROSS  = /opt/gcc-arm-none-eabi-4_8-2014q3/bin/arm-none-eabi-
LD     = $(CROSS)ld
CC     = $(CROSS)gcc
OBJCPY = $(CROSS)objcopy

CFLAGS = -Wall -Wextra -O2 -mthumb -mcpu=cortex-m4 -mfpu=fpv4-sp-d16 -mfloat-abi=softfp

RUSTFLAGS  = -O -L lib/ --target $(TARGET)
RUSTFLAGS += -C lto -C target-cpu=cortex-m4 -C relocation-model=static

.SUFFIXES: .o .rs .c

all: boot.bin

# Need to add -C no-split-stack when it lands
.rs.o:
	$(RUSTC) $(RUSTFLAGS) --emit obj -o $@ $<

boot.elf: boot.ld main.o
	$(LD) --gc-sections -o $@ -T $^

boot.bin: boot.elf
	$(OBJCPY) -O binary $< $@

.PHONY: clean

clean:
	rm -f *.o boot.bin boot.elf
