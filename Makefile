DOCKER_NAME ?= rcore-tutorial-v3
.PHONY: docker build_docker
	
docker:
	docker run --rm -it -v ${PWD}:/mnt -w /mnt ${DOCKER_NAME} bash

build_docker: 
	docker build -t ${DOCKER_NAME} .

bash:
	docker start rcore-tutorial
	docker exec -it rcore-tutorial bash

fmt:
	cd os ; cargo fmt;  cd ..

