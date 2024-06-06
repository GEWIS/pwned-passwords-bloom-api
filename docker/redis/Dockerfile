FROM redis/redis-stack-server:7.2.0-v10

# Copy the last database dump.
COPY ./data/dump.rdb /data/dump.rdb

CMD ["/entrypoint.sh"]
