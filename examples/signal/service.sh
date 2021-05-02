set -e

sudo apt update
sudo apt install -y git maven redis-server
sed -i '/# cluster-enabled yes/c\cluster-enabled yes' /etc/redis/redis.conf
systemctl restart redis
mkdir -p /opt/signal
cd /opt/signal
git clone https://github.com/signalapp/Signal-Server.git
cd Signal-Server
git checkout v5.31
mvn -DskipTests package

# cd /opt/signal/Signal-Server/service/target
# java -jar TextSecureServer-5.31.jar server /etc/nocloud/context/config.yml
