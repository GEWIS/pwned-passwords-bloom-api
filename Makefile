.PHONY: all build buildapi buildredis login push pushapi pushredis
.DEFAULT_GOAL := all

SHELL = /bin/bash

all: build login push

build: buildapi buildredis

buildapi:
	@docker build -t abc.docker-registry.gewis.nl/web/pwned-passwords/api:latest -f docker/api/Dockerfile .

buildredis:
	@docker build -t abc.docker-registry.gewis.nl/web/pwned-passwords/redis:latest -f docker/redis/Dockerfile .

login:
	@docker login abc.docker-registry.gewis.nl

push: pushapi pushredis

pushapi:
	@docker push abc.docker-registry.gewis.nl/web/pwned-passwords/api:latest

pushredis:
	@docker push abc.docker-registry.gewis.nl/web/pwned-passwords/redis:latest