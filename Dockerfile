####################################################################################################
## Final image
####################################################################################################
FROM alpine:latest
COPY . .


RUN chown 755 ./target/release/chatr

CMD ["./target/release/chatr"]