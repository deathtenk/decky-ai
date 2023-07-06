.PHONY: all setup py_deps

ROOT_DIR := $(PWD)

all: setup py_deps

setup:
	pip install -r requirements.txt
py_deps:
	cd .env/lib/python3.10/site-packages/ && cp -r requests certifi charset_normalizer idna urllib3 $(ROOT_DIR)/defaults/py_deps/
