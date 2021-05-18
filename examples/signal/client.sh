set -e

sudo apt update
sudo apt install -y git openjdk-14-jdk
mkdir -p /opt/signal
cd /opt/signal
git clone https://github.com/AsamK/signal-cli.git
cd signal-cli
git checkout 5c3fc44d00cb7b18c4c5b4b6b5d7fe09f18973db
git apply --ignore-space-change --ignore-whitespace /etc/nocloud/context/use_custom_server.patch
./gradlew build
./gradlew installDist

# cd /opt/signal/signal-cli/build/install/signal-cli/bin
# ./signal-cli -u +447722000001 register
# ./signal-cli -u +447722000001 verify 111111
