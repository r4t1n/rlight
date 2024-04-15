target = target/release/rlight
target_path = /usr/local/bin/rlight
target_dir = target/

udev_rules = 90-rlight.rules
udev_rules_path = /lib/udev/rules.d/90-rlight.rules

build:
	cargo build --release

check-root:
	@if [ `id -u` -ne 0 ]; then \
		echo "Root privileges needed"; \
		exit 1; \
	fi

clean:
	rm -rf $(target_dir) 

install: check-root
	cp $(target) $(target_path)
	cp $(udev_rules) $(udev_rules_path)

uninstall: check-root
	rm -f $(target_path) $(udev_rules_path)
