#!/bin/bash

set -e
temp_dir=$(mktemp -d /tmp/keypair.XXXXXXX)
cd $temp_dir
wget https://www.bouncycastle.org/download/bcprov-jdk15to18-168.jar
wget -O GenerateKeyPairEC.java https://github.com/DP-3T/dp3t-sdk-backend/raw/100bcde00df2326a1c5cc9588177595836ecc512/GenerateKeyPairEC.java
javac -cp "./bcprov-jdk15to18-168.jar" GenerateKeyPairEC.java
java -cp "./bcprov-jdk15to18-168.jar:./" GenerateKeyPairEC

printf "Public:\n"
cat generated_pub.pem

printf "\n\nPrivate:\n"
cat generated_private.pem
printf "\n"
