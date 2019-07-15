TARGET:=blue-pill
OUTPUT_DIR:=target/thumbv7m-none-eabi/release/

.PHONY: always

.PHONY: all
all: build-release

.PHONY: build-release
build-release: $(OUTPUT_DIR)/$(TARGET).bin

.PHONY: upload
upload: $(OUTPUT_DIR)/$(TARGET).bin
	./bin/st-flash write $< 0x8000000

.PHONY: erase
erase:
	./bin/st-flash erase

$(OUTPUT_DIR)/$(TARGET): always
	@cargo build --release

$(OUTPUT_DIR)/$(TARGET).bin: $(OUTPUT_DIR)/$(TARGET)
	arm-none-eabi-objcopy -O binary $< $@
