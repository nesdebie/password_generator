TARGET = password_generator
SRC = main.rs

all: $(TARGET)

$(TARGET):
	@if [ -f $(TARGET) ] && [ ! -d $(TARGET) ]; then \
		echo "Removing existing file '$(TARGET)' to make room for Cargo project..."; \
		rm -f $(TARGET); \
	fi
	@if [ ! -f $(TARGET)/Cargo.toml ]; then \
		echo "Creating new Cargo project..."; \
		cargo new $(TARGET) --bin; \
		cat ./dependencies >> $(TARGET)/Cargo.toml; \
	fi
	cp $(SRC) $(TARGET)/src/
	cd $(TARGET) && cargo build
	@echo "Build complete. Run 'make run' to execute the program."

run:
	cd $(TARGET) && cargo run

clean:
	rm -rf $(TARGET)

re: clean all

.PHONY: all clean re run