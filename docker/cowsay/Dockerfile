FROM ubuntu:latest

RUN apt update && apt install -y cowsay fortune
ADD entrypoint.sh /

ENTRYPOINT ["./entrypoint.sh"]
