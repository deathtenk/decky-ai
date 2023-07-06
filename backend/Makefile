# This is the default target, which will be built when
# you invoke make
.PHONY: all build clean
all: build

# This rule tells make how to build hello from hello.cpp
build:
	mkdir -p ./out
	cp ./aichat/target/x86_64-unknown-linux-gnu/release/aichat ./out/

clean:
	rm -rf ./aichat/target
