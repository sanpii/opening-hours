YARN?=yarn
YARN_FLAGS?=
TRUNK?=trunk
TRUNK_FLAGS?=

.DEFAULT_GOAL := build

ifeq ($(APP_ENVIRONMENT),prod)
	YARN_FLAGS+=--production
	TRUNK_FLAGS+=--release
endif

ifneq (,$(wildcard ./.env))
	include .env
	export
endif

build: yarn
	RUSTFLAGS='--cfg getrandom_backend="wasm_js"' $(TRUNK) build $(TRUNK_FLAGS)
.PHONY: build

yarn: yarn.lock
.PHONY: yarn

yarn.lock: package.json
	$(YARN) $(YARN_FLAGS) install

serve: build
	$(TRUNK) serve $(TRUNK_FLAGS)
.PHONY: serve
