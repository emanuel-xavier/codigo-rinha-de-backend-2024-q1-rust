#!/usr/bin/bash

# Use este script para executar testes locais

RESULTS_WORKSPACE="$(pwd)/user-files/results"
GATLING_BIN_DIR=$HOME/gatling/bin
GATLING_WORKSPACE="$(pwd)/user-files"

runGatling() {
    sh $GATLING_BIN_DIR/gatling.sh -rm local -s RinhaBackendCrebitosSimulation \
        -rd "Rinha de Backend - 2024/Q1: Crébito" \
        -rf $RESULTS_WORKSPACE \
        -sf "$GATLING_WORKSPACE/simulations"
}

startContainers() {
    docker compose up -d
}

stopContainers() {
    docker compose down
}

startTest() {
    for i in {1..20}; do
        # 2 requests to wake the 2 api instances up :)
        curl --fail http://localhost:9999/clientes/1/extrato && \
        echo "" && \
        curl --fail http://localhost:9999/clientes/1/extrato && \
        echo "" && \
        runGatling && \
        break || sleep 2;
    done
}

#startContainers
startTest
#stopContainers
