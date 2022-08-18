DOCKER_REPOSITORY=martinabeleda/athena
DOCKER_TAG=latest
DOCKER_IMAGE=${DOCKER_REPOSITORY}:${DOCKER_TAG}

build:
	docker build -t ${DOCKER_IMAGE} .

run:
	docker run -p 8080:8080 ${DOCKER_IMAGE}
