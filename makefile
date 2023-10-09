db:
	docker-compose -f docs/postgres_local/docker-compose.yaml up -d

sql:
	diesel migration generate $(name)

mig:
	diesel migration run

test:
	make mig
	sleep 3
	cargo test --verbose
