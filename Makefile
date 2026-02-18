.PHONY: serve release wasm upload build archive profile build-release patch

serve:
	bevy serve web

build:
	bevy build --yes web --bundle

build-release:
	bevy build --release --yes web --bundle

USERNAME = rockcen
PROJECT = just-let-me-sleep
BUILD_DIR = wasm_builds
DIST_DIR = target/bevy_web/web-release/bevy_game
CHANNEL = html5
#last tag name, fallback to v1.0.0 if no tags exist
VERSION := $(shell git describe --tags --abbrev=0 2>/dev/null || echo "v1.0.0")
ZIP_FILE := $(BUILD_DIR)/$(PROJECT)-$(VERSION).zip

colon = :

upload: build-release archive
	butler push $(ZIP_FILE) $(USERNAME)/$(PROJECT)$(colon)$(CHANNEL) --userversion $(VERSION)

archive: $(DIST_DIR)
	mkdir -p $(BUILD_DIR)
	zip -r $(ZIP_FILE) $(DIST_DIR)

$(DIST_DIR):
	@echo "Dist folder does not exist"

# minor or release major, minor, patch,
patch:
	cargo release patch --no-publish --execute


profile:
	cargo bloat -n 100000 --message-format json > out.json
