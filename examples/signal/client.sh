set -e

sudo apt update
sudo apt install -y git openjdk-14-jdk
mkdir -p /opt/signal
cd /opt/signal
git clone https://github.com/AsamK/signal-cli.git
cd signal-cli
git apply --ignore-space-change --ignore-whitespace /etc/nocloud/context/use_custom_server.patch
./gradlew build
./gradlew installDist

# cd /opt/signal/signal-cli/build/install/signal-cli/bin
