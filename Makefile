build:
	cd todayiwill
	@sed -i "s/^version =.*/version = \"$(VERSION)\"/g" Cargo.toml
	cargo build --release

build-doc:
	cd todayiwill
	cargo doc --no-deps
	@echo '<meta http-equiv="refresh" content="0;url=todayiwill/index.html">' > target/doc/index.html

build-deb:
	fpm -s dir \
		-t deb \
		-n todayiwill \
		-v $(VERSION) \
		--license LICENSE \
		--description "A CLI reminder app that offers a simple yet powerful solution to enhance productivity and ensure that you stay on top of your daily responsibilities" \
		--maintainer "Vinicius Mayrink <vncsmyrnk@gmail.com>" \
		todayiwill/target/release/todayiwill=/usr/bin/todayiwill
