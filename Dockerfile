FROM alpine:3.10
#RUN apk add libseccomp
COPY target/x86_64-unknown-linux-musl/release/generust-example-project .
EXPOSE 5500/tcp
ENTRYPOINT ["/generust-example-project"]
