.PHONY: all build buildapi buildnginx buildredis login push pushapi pushnginx pushredis
.DEFAULT_GOAL := all

SHELL = /bin/bash

all: build login push

build: buildapi buildnginx buildredis

buildapi:
	@docker build -t abc.docker-registry.gewis.nl/web/pwned-passwords/api:latest -f docker/api/Dockerfile .

buildnginx:
	@docker build -t abc.docker-registry.gewis.nl/web/pwned-passwords/nginx:latest -f docker/nginx/Dockerfile .

buildredis:
	@docker build -t abc.docker-registry.gewis.nl/web/pwned-passwords/redis:latest -f docker/redis/Dockerfile .

login:
	@docker login abc.docker-registry.gewis.nl

push: pushapi pushnginx pushredis

pushapi:
	@docker push abc.docker-registry.gewis.nl/web/pwned-passwords/api:latest

pushnginx:
	@docker push abc.docker-registry.gewis.nl/web/pwned-passwords/nginx:latest

pushredis:
	@docker push abc.docker-registry.gewis.nl/web/pwned-passwords/redis:latest