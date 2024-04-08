program = rlight
target = target/release/$(program)
target_path = /usr/local/bin/

udev_rules = 90-rlight.rules
udev_rules_path = /lib/udev/rules.d/

build:
	cargo build --release

check-root:
	@if [ `id -u` -ne 0 ]; then \
		echo "Root privileges needed"; \
		exit 1; \
	fi

install: check-root
	cp $(target) $(target_path)
	cp $(udev_rules) $(udev_rules_path)

uninstall: check-root
	rm $(target_path)/$(program) $(udev_rules_path)/$(udev_rules)