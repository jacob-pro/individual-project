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

# Usage
# cd /opt/signal/signal-cli/build/install/signal-cli/bin
# USER=$(/etc/nocloud/env.sh number)
# ./signal-cli -u $USER register
# ./signal-cli -u $USER verify $(/etc/nocloud/env.sh code)
# ./signal-cli -u $USER send -m "message..." DESTINATION
# ./signal-cli --verbose -u $USER receive
