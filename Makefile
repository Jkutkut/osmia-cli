# ****** Operating System ******
OS = $(shell uname -s)
ifeq ($(OS),Linux)
	DIR = $(shell pwd)
endif
ifeq ($(OS),Darwin)
	DIR = ${PWD}
endif
REPO = $(shell echo ${DIR} | sed 's/.*\///' | tr '[:upper:]' '[:lower:]')

# ****** Rust Constants ******
CARGO = /root/.cargo/bin/cargo
CODE_VOLUME = -v ${DIR}:/${REPO}
CARGO_REGISTRY = -v cargo_registy:/root/.cargo/registry

# ****** Docker Constants ******
DOCKER_RUN = docker run --rm -t
DOCKER_RUN_IT = ${DOCKER_RUN} -it --name ${REPO}

RUN_ATTRS = ${CODE_VOLUME} ${CARGO_REGISTRY} -w /${REPO}

all: build

terminal:
	${DOCKER_RUN_IT} ${RUN_ATTRS} jkutkut/docker4rust

reset_file_permissions:
	sudo chown -R ${USER}:${USER} .

build:
	${DOCKER_RUN} ${RUN_ATTRS} --entrypoint cargo jkutkut/docker4rust build

build_release:
	${DOCKER_RUN} ${RUN_ATTRS} --entrypoint cargo jkutkut/docker4rust build --release

run:
	${DOCKER_RUN} ${RUN_ATTRS} --entrypoint cargo jkutkut/docker4rust run

run_backtrace:
	${DOCKER_RUN} ${RUN_ATTRS} -e RUST_BACKTRACE=1 --entrypoint cargo jkutkut/docker4rust run

test: build
	${DOCKER_RUN} ${RUN_ATTRS} --entrypoint cargo jkutkut/docker4rust test

test_release: build_release
	${DOCKER_RUN} ${RUN_ATTRS} --entrypoint cargo jkutkut/docker4rust test --release

test_backtrace: build
	${DOCKER_RUN} ${RUN_ATTRS} -e RUST_BACKTRACE=1 --entrypoint cargo jkutkut/docker4rust test

doc:
	${DOCKER_RUN} ${RUN_ATTRS} --entrypoint cargo jkutkut/docker4rust doc --lib --examples

clean:
	${DOCKER_RUN} ${RUN_ATTRS} --entrypoint cargo jkutkut/docker4rust clean

install: build_release
	cp target/release/${REPO} /usr/bin/osmia
