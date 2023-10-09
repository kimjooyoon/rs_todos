db:
	docker-compose -f docs/postgres_local/docker-compose.yaml up -d

sql:
	diesel migration generate $(name)

mig:
	diesel migration run

test:
	sleep 2
	make mig
	sleep 2
	cargo test --verbose
