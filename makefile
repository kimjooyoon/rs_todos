db:
	docker-compose -f docs/postgres_local/docker-compose.yaml up -d

sql:
	diesel migration generate $(name)

mig:
	diesel migration run
