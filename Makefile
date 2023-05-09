binname="raachat_bot"


help:
	@echo 'there is no help.. yet'

run: release size
	@./target/release/$(binname)

edit:
	@nvim ./src/main.rs

savetogit:
	@git add . && git commit -m 'saving' && git push

release:
	@cargo rustc --release -- -C prefer-dynamic

test:
	@cargo test

size:
	@ls -lAh ./target/release/$(binname)

clean:
	@cargo clean
