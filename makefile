# Variables
VERSION := $(shell cargo pkgid | cut -d\# -f2 | cut -d: -f2)
FORCE_TAG := false

.PHONY: release repo-update 

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

# Get the current date
DATE := $(shell date +%Y-%m-%d)

# Import commit types from existing configuration
define COMMIT_TYPES
feat:     A new feature
fix:      A bug fix
docs:     Documentation only changes
style:    Changes that do not affect the meaning of the code
refactor: A code change that neither fixes a bug nor adds a feature
perf:     A code change that improves performance
test:     Adding missing tests or correcting existing tests
build:    Changes that affect the build system or external dependencies
ci:       Changes to CI configuration files and scripts
chore:    Other changes that don't modify src or test files
revert:   Reverts a previous commit
endef
export COMMIT_TYPES

AVAILABLE_FOLDERS := src tests

repo-update:
	@echo "Available folders: $(AVAILABLE_FOLDERS)"
	@echo ""
	@echo "Examples:"
	@echo "  • Press enter to commit all folders"
	@echo "  • Type 'gridwalk-backend' to commit only backend"
	@echo "  • Type 'gridwalk-backend gridwalk-ui' to commit backend and UI"
	@echo ""
	@read -p "Enter the names of the folders you wish to update (space-separated, or just hit enter to update all): " folders; \
	if [ -z "$$folders" ]; then \
		make git-add-all git-commit git-push; \
	else \
		make git-add-selected FOLDERS="$$folders" git-commit git-push; \
	fi

git-add-all:
	git add .

git-add-selected:
	@for folder in $(FOLDERS); do \
		if [[ " $(AVAILABLE_FOLDERS) " =~ " $$folder " ]]; then \
			echo "Adding folder: $$folder"; \
			git add $$folder/.; \
		else \
			echo "Warning: $$folder is not a recognized folder"; \
		fi \
	done

git-commit:
	@echo "Available commit types:"
	@echo "$$COMMIT_TYPES" | sed 's/^/  /'
	@echo
	@read -p "Enter commit type: " type; \
	if echo "$$COMMIT_TYPES" | grep -q "^$$type:"; then \
		read -p "Enter commit scope (optional, press enter to skip): " scope; \
		read -p "Is this a breaking change? (y/N): " breaking; \
		read -p "Enter commit message: " msg; \
		if [ "$$breaking" = "y" ] || [ "$$breaking" = "Y" ]; then \
			if [ -n "$$scope" ]; then \
				git commit -m "$$type!($$scope): $$msg [$(DATE)]" -m "BREAKING CHANGE: $$msg"; \
			else \
				git commit -m "$$type!: $$msg [$(DATE)]" -m "BREAKING CHANGE: $$msg"; \
			fi; \
		else \
			if [ -n "$$scope" ]; then \
				git commit -m "$$type($$scope): $$msg [$(DATE)]"; \
			else \
				git commit -m "$$type: $$msg [$(DATE)]"; \
			fi; \
		fi; \
	else \
		echo "Invalid commit type. Please use one of the available types."; \
		exit 1; \
	fi

git-push:
	git push
