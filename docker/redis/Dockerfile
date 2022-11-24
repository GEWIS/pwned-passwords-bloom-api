FROM redislabs/rebloom:2.4.3 as rebloom
FROM redis:7.0-alpine3.16

# Ensure that the correct timezone is present.
RUN apk add --no-cache --virtual .build-deps \
        tzdata \
    && cp /usr/share/zoneinfo/Europe/Amsterdam /etc/localtime \
    && echo 'Europe/Amsterdam' > /etc/timezone \
    && apk del .build-deps

# Copy the RedisBloom library from the `rebloom` image to our own image.
COPY --from=rebloom /usr/lib/redis/modules/redisbloom.so /usr/lib/redis/modules/redisbloom.so

# Copy the last database dump.
COPY ./data/dump.rdb /data/dump.rdb

CMD ["redis-server", "--loadmodule", "/usr/lib/redis/modules/redisbloom.so"]
