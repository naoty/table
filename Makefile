VERSION ?= $$(git describe --tags)

default: install

clean:
	rm bin/table
	rm $(GOPATH)/bin/table

build:
	go build -ldflags "-X main.Version=$(VERSION)" -o bin/table

install:
	go install -ldflags "-X main.Version=$(VERSION)"
