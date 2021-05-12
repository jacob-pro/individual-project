# Guide to setup Signal Server
# Must be on an Ubuntu AWS Instance

# Copy config file
scp .\config.yml ubuntu@${SIGNAL_IP}:/home/ubuntu/

# Download and build server
sudo su
apt update
apt install -y git maven
sed -i '/# cluster-enabled yes/c\cluster-enabled yes' /etc/redis/redis.conf
systemctl restart redis
mkdir -p /opt/signal
cd /opt/signal
git clone https://github.com/signalapp/Signal-Server.git
cd Signal-Server
mvn -DskipTests package

# Install Docker + Dependencies
https://docs.docker.com/engine/install/ubuntu/
apt install docker-compose
cd /opt/signal
wget https://raw.githubusercontent.com/madeindra/signal-setup-guide/master/signal-server-5.xx/docker-compose.yml
docker-compose up -d signal_database
docker-compose up -d redis_cluster

# Run Server
cd /opt/signal/Signal-Server/service/target
java -jar TextSecureServer-5.80.jar server /home/ubuntu/config.yml
