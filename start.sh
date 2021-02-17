#!/bin/bash
docker stop yunchao_plus_wallet
docker rm yunchao_plus_wallet
docker run -d --name=yunchao_plus_wallet -p 8081:8080 -v ~/wallet/logs:/app/logs yunchao_plus_wallet
