# Variables
VERSION := $(shell cargo pkgid | cut -d\# -f2 | cut -d: -f2)
FORCE_TAG := false

.PHONY: release

release:
	@echo "Creating release $(VERSION)..."
	@if [ -z "$(VERSION)" ]; then echo "Error: VERSION is not set"; exit 1; fi

	# Check if tag exists
	@if git rev-parse v$(VERSION) >/dev/null 2>&1; then \
		if [ "$(FORCE_TAG)" = "true" ]; then \
			echo "Updating tag v$(VERSION)..."; \
			git tag -fa v$(VERSION) -m "Release version $(VERSION)"; \
			git push -f origin v$(VERSION); \
		else \
			echo "Tag v$(VERSION) exists. Use FORCE_TAG=true to override."; \
			exit 1; \
		fi; \
	else \
		echo "Creating tag v$(VERSION)..."; \
		git tag -a v$(VERSION) -m "Release version $(VERSION)"; \
		git push origin v$(VERSION); \
	fi

	# Get release notes
	@echo "Enter release notes (Ctrl+D to finish):"
	@NOTES=$$(cat); \
	gh release create v$(VERSION) --title "Nebby v$(VERSION)" --notes "$$NOTES" \
		$$(if [ "$(FORCE_TAG)" = "true" ]; then echo "--target main"; fi) || \
		{ echo "Error: Failed to create GitHub release."; exit 1; }

	@echo "Release v$(VERSION) published on GitHub"
