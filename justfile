alias r := run

default: run

run:
	cargo run

gen-certs:
	cd certificates; openssl req -nodes -x509 -subj "/C=US/ST=Denial/L=Springfield/O=Dis/CN=www.example.com" -newkey rsa:4096 -keyout key.pem -out cert.pem -sha256