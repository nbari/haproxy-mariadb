.PHONY: all add build down ps clean stop ha

all:
	@mkdir -p mariadb/db
	docker-compose up

down:
	docker-compose down

clean:
	@find . -type d -name "db" -execdir sh -c 'rm -rf {}/*' \;
