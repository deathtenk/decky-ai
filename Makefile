.PHONY: all setup py_deps link

ROOT_DIR := $(PWD)

all: setup py_deps link

setup:
	pip install -r requirements.txt
py_deps:
	mkdir $(ROOT_DIR)/defaults/py_deps && cd $(ROOT_DIR)/.env/lib/python3.10/site-packages/ && cp -r requests certifi charset_normalizer idna urllib3 $(ROOT_DIR)/defaults/py_deps/
link:
	ln -s $(ROOT_DIR)/defaults/py_deps $(ROOT_DIR)/py_deps
