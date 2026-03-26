#!/bin/bash
set -a
source .env
set +a
envsubst '$SERVER_IP $DOMAIN' < nginx.conf.template > nginx.conf