# Makefile for Nebby GitHub Release and Tagging

# Variables
VERSION := $(shell cargo pkgid | cut -d\# -f2 | cut -d: -f2)

.PHONY: release
release:
	@echo "Creating release $(VERSION)..."
	@if [ -z "$(VERSION)" ]; then echo "Error: VERSION is not set"; exit 1; fi

	# Create a new tag
	git tag -a v$(VERSION) -m "Release version $(VERSION)"
	git push origin v$(VERSION)

	# Prompt for release notes
	@echo "Please enter release notes (press Enter twice when finished):"
	@NOTES=$$(cat <<EOF

	EOF
	); \
	gh release create v$(VERSION) \
		--title "Nebby v$(VERSION)" \
		--notes "$$NOTES"

	@echo "Release v$(VERSION) created and published on GitHub"
