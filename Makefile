start=$(shell grep -n 'CACHE:' web/opening-hours.manifest  | cut -d: -f1)
end=$(shell grep -n 'NETWORK:' web/opening-hours.manifest  | cut -d: -f1)
files=$(shell head -$(shell expr $(end) - 1) web/opening-hours.manifest | tail -$(shell expr $(end) - $(start) - 1) | sed 's%^%web/%')
sources=web/manifest.webapp web/opening-hours.manifest $(files)

all: opening-hours.zip opening-hours.apk

opening-hours.zip: $(sources)
	cd web && zip ../$@ $(shell echo $^ | sed 's%web/%%g')

opening-hours.apk: opening-hours.zip
	./node_modules/.bin/mozilla-apk-cli $^ $@

clean:
	rm -f opening-hours.zip opening-hours.apk
