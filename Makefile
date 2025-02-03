# DB Variables
POSTGRES_IMAGE := postgres:17.2-alpine
POSTGRES_CONTAINER := wad_db
DB_NAME := wad_db_local
DB_USER := wad_user
DB_PASSWORD := wad_password
DB_PORT := 5432
DATABASE_URL := postgresql://wad_user:wad_password@localhost:5432/wad_db_local

db-pull:
	docker pull $(POSTGRES_IMAGE)

db-start:
	docker run --name $(POSTGRES_CONTAINER) \
		-e POSTGRES_PASSWORD=$(DB_PASSWORD) \
		-e POSTGRES_USER=$(DB_USER) \
		-e POSTGRES_DB=$(DB_NAME) \
		-p $(DB_PORT):5432 -d $(POSTGRES_IMAGE)

db-reset:
	@echo "Stopping and removing the existing database container..."
	-docker stop $(POSTGRES_CONTAINER)
	-docker rm $(POSTGRES_CONTAINER)

	@make db-start
	@sleep 5
	@make db-run-migrations
	@make run-seed
	@echo "Database reset complete!"

db-add-migration:
	sqlx migrate add -r $(MIGRATION_NAME) --source ./database/migrations

db-run-migrations:
	sqruff fix database/migrations
	sqruff lint database/migrations
	sqlx migrate run --source database/migrations --database-url $(DATABASE_URL)

# Applications Bin Commands

run-server:
	DATABASE_URL=$(DATABASE_URL) cargo run --bin server

run-client:
	DATABASE_URL=$(DATABASE_URL) cargo run --bin client

run-seed:
	DATABASE_URL=$(DATABASE_URL) cargo run --bin seed 