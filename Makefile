VERSION ?= $$(git describe --tags --always)

default: install

clean:
	rm $(GOPATH)/bin/table

install:
	go install -ldflags "-X main.Version=$(VERSION)"
