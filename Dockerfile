FROM ubuntu:latest
LABEL authors="akdem"

ENTRYPOINT ["top", "-b"]