BOWER_FLAGS=
COMPOSER_FLAGS=--no-interaction

ifeq ($(APP_ENVIRONMENT),prod)
	BOWER_FLAGS+=--production
	COMPOSER_FLAGS+=--prefer-dist --no-dev --classmap-authoritative
endif

TASKS=
ifneq ("$(wildcard composer.json)","")
	TASKS+=vendor
endif

ifneq ("$(wildcard bower.json)","")
	TASKS+=assets
endif

all: $(TASKS)

vendor: composer.lock

composer.lock: composer.json
	composer install $(COMPOSER_FLAGS)

assets: web/lib

web/lib: bower.json
	bower install $(BOWER_FLAGS)

distclean:
	rm -rf vendor composer.lock web/lib

.PHONY: all assets distclean
