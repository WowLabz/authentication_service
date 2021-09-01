FROM rust:1.54

WORKDIR /authentication_service
COPY . /authentication_service/

RUN cargo build 
EXPOSE 7001
CMD cargo run
