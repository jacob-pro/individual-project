set -e

SIGNAL_SERVER="http://3.8.29.164:8080/"

sudo apt update
sudo apt install -y git default-jdk
mkdir -p /opt/signal
cd /opt/signal
git clone https://github.com/AsamK/signal-cli.git
cd signal-cli
sed -i "s#https://textsecure-service.whispersystems.org#${SIGNAL_SERVER}#" \
./lib/src/main/java/org/asamk/signal/manager/config/LiveConfig.java
./gradlew build
./gradlew installDist

# cd /opt/signal/signal-cli/build/install/signal-cli/bin
