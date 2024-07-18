FROM ubuntu:jammy

RUN apt-get update && \
    apt-get install build-essential -y && \
    apt-get install cmake -y && \
    apt install git -y && \
    apt install curl -y && \
    apt install pkg-config -y && \
    apt-get install libssl-dev -y && \
    apt-get install unzip -y && \
    apt-get install protobuf-compiler -y && \
    curl https://sh.rustup.rs -sSf | bash -s -- -y && \
    apt install python3 -y && \
    apt install python3-pip -y && \
    pip install grpcio grpcio-tools mypy-protobuf && \
    apt install nodejs npm -y && \
    apt install libpq-dev -y


RUN  cd /opt && \
     curl -LO https://go.dev/dl/go1.20.11.linux-amd64.tar.gz && \
     tar -xvzf go1.20.11.linux-amd64.tar.gz && \
     mv go /usr/local 

ENV PATH $PATH:/usr/local/go/bin:/root/go
ENV PATH="/root/.cargo/bin:${PATH}"
RUN go env -w GO111MODULE=on
RUN mkdir -p /opt/rust-cld2 && \
    go install google.golang.org/protobuf/cmd/protoc-gen-go@latest  && \
    go install google.golang.org/grpc/cmd/protoc-gen-go-grpc@latest


ENTRYPOINT ["tail", "-f", "/dev/null"]