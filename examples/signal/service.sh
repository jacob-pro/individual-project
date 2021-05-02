set -e

sudo apt update
sudo apt install -y git maven
mkdir -p /opt/signal
cd /opt/signal
git clone https://github.com/signalapp/Signal-Server.git
cd Signal-Server/service
mvn install -DskipTests

