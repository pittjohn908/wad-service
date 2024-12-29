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

db-run-migrations:
	sqruff fix migrations
	sqruff lint migrations
	sqlx migrate run --database-url $(DATABASE_URL)