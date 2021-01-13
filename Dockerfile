FROM ubuntu:latest

COPY bin/ ./

RUN apt-get update \
    && apt-get install -y --no-install-recommends rustc \
    && chmod +x ./main.rs \
    && rustc ./main.rs

CMD "./main"
