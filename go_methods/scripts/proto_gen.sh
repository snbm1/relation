#!/bin/bash 

set -e 

echo "Converting from proto to go..."

RED='\033[0;31m'
YELLOW='\033[1;33m'
GREEN='\033[0;32m'
NC='\033[0m'

if ! command -v protoc &> /dev/null; then 
    echo -e "${RED} protoc not found!${NC}"
    exit 1 
fi 

if ! command -v protoc-gen-go &> /dev/null; then
    echo -e "${YELLOW} protoc-gen-go not found, installing...${NC}"
    go install google.golang.org/protobuf/cmd/protoc-gen-go@latest
    export PATH="$PATH:$(go env GOPATH)/bin"
fi 

echo -e "${YELLOW} Cleaning old generated files...${NC}"
cd .. 
cd relationrpc 
rm -f *.pb.go 

echo -e "${YELLOW}Generating from proto files...${NC}"
protoc \
    --proto_path=. \
    --go_out=. \
    --go_opt=paths=source_relative \
    ./*.proto


if [ $? -eq 0 ]; then
    echo -e "${GREEN} Successfully generated:${NC}"
    ls ./*.pb.go | awk '{print "  " $9}'

    cd .. 
    echo -e "${YELLOW} Updating go modules...${NC}"
    go mod tidy 
    echo -e "${GREEN} Done! Protobuf code generated in relationrpc${NC}"
else 
    echo -e "${RED} Generation failed!${NC}"
    exit 1
fi