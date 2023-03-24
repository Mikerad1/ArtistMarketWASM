# Makefile for thehaven_auth

PROJECT  = thehaven_auth
VERSION  = $(shell cargo metadata --no-deps --format-version 1 | jq -r '.packages[] .version' | head -1)
REVISION = 0
# list of all contract claims for actor signing (space-separated)
CLAIMS   = michaelrademeyer:jwt_handler_provider wasmcloud:sqldb
# registry url for our actor
REG_URL  = localhost:5000/v2/$(PROJECT):$(VERSION)
# command to upload to registry (without last wasm parameter)
PUSH_REG_CMD = wash reg push --insecure $(REG_URL)

# friendly name for the actor
ACTOR_NAME = "thehaven_auth"
# optional call alias for actor
ACTOR_ALIAS=the_haven/auth

include ./actor.mk

test::
	cargo clippy --all-features --all-targets
