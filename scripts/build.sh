#!/usr/bin/env bash

docker build --platform linux/amd64 --tag zero2prod --file Dockerfile . --load