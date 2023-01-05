.PHONY: all build buildapi buildnginx buildredis login push pushapi pushnginx pushredis
.DEFAULT_GOAL := all

SHELL = /bin/bash

all: build login push

build: buildapi buildnginx buildredis

buildapi:
	@docker build -t web.docker-registry.gewis.nl/pwned-passwords_api:latest -f docker/api/Dockerfile .

buildnginx:
	@docker build -t web.docker-registry.gewis.nl/pwned-passwords_nginx:latest -f docker/nginx/Dockerfile .

buildredis:
	@docker build -t web.docker-registry.gewis.nl/pwned-passwords_redis:latest -f docker/redis/Dockerfile .

login:
	@docker login web.docker-registry.gewis.nl

push: pushapi pushnginx pushredis

pushapi:
	@docker push web.docker-registry.gewis.nl/pwned-passwords_api:latest

pushnginx:
	@docker push web.docker-registry.gewis.nl/pwned-passwords_nginx:latest

pushredis:
	@docker push web.docker-registry.gewis.nl/pwned-passwords_redis:latest