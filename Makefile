DOCKER_IMAGE_PREFIX=emanueljesusxavier1999
DOCKER_IMAGE_NAME=rinha-2024-q1-rust

.PHONY: build exec

build:
	cargo build --release

run:
	cargo run

docker-build-images:
	docker buildx build -t $(DOCKER_IMAGE_PREFIX)/$(DOCKER_IMAGE_NAME) .
	docker buildx build --platform linux/amd64 -t $(DOCKER_IMAGE_PREFIX)/$(DOCKER_IMAGE_NAME)-amd64 .

docker-push-images:
	docker push $(DOCKER_IMAGE_PREFIX)/$(DOCKER_IMAGE_NAME) 
	docker push $(DOCKER_IMAGE_PREFIX)/$(DOCKER_IMAGE_NAME)-amd64

docker-run: docker-clean
	docker network create rinha
	docker run --name db -v ./db/script.sql:/docker-entrypoint-initdb.d/script.sql --network rinha -e POSTGRES_PASSWORD=1234 -d --restart=always --health-cmd="pg_isready -U postgres" --health-interval=10s --health-retries=5 --health-start-period=30s postgres
	docker run --name rinha --network rinha -e DATABA_URL="postgres://postgres:1234@db/postgres" -e PORT="9999" --restart unless-stopped -p 9999:9999 -d $(DOCKER_IMAGE_PREFIX)/$(DOCKER_IMAGE_NAME)

docker-start:
	-docker start db
	-docker start rinha

docker-stop:
	-docker stop db
	-docker stop rinha

docker-clean:
	-docker images --filter "dangling=true" --filter "reference=$(DOCKER_IMAGE_PREFIX)/$(DOCKER_IMAGE_NAME)*" --format "{{.ID}}" | xargs docker rmi
	-docker rm -f db rinha
	-docker network rm rinha
