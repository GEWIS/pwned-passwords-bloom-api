FROM nginx:stable-alpine
# Ensure that `www-data` actually exists.
RUN adduser -D -H -u 1000 -s /bin/sh www-data -G www-data

# Ensure that the correct timezone is present.
RUN apk add --no-cache --virtual .build-deps \
        tzdata \
    && cp /usr/share/zoneinfo/Europe/Amsterdam /etc/localtime \
    && echo 'Europe/Amsterdam' > /etc/timezone \
    && apk del .build-deps

COPY --chown=www-data:www-data ./docker/nginx/nginx.conf /etc/nginx/nginx.conf

CMD ["/bin/sh", "-c", "exec nginx -g 'daemon off;'"]
