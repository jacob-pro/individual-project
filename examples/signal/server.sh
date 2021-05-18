# Guide to setup Signal Server
# Must be on an Ubuntu AWS Instance
# AWS Instance must have an IAM role
# Role must access a deployed AWS AppConfig with Environment

# Copy config files
scp ./config.yml ubuntu@${SIGNAL_IP}:/home/ubuntu/
scp ./nginx.conf ubuntu@${SIGNAL_IP}:/home/ubuntu/

# Download and build server
sudo su
apt update
apt install -y git maven
mkdir -p /opt/signal
cd /opt/signal
git clone https://github.com/signalapp/Signal-Server.git
cd Signal-Server
git checkout v4.97
mvn -DskipTests package

# Install Docker + Dependencies
https://docs.docker.com/engine/install/ubuntu/
cd /opt/signal
docker run -d --name accountdb -e "POSTGRES_PASSWORD=postgres" -e "POSTGRES_DB=accountdb" -p 5432:5432 postgres:11
docker run -d --name abusedb -e "POSTGRES_PASSWORD=postgres" -e "POSTGRES_DB=abusedb" -p 5433:5432 postgres:11
docker run -d --name messagedb -e "POSTGRES_PASSWORD=postgres" -e "POSTGRES_DB=messagedb" -p 5434:5432 postgres:11

docker run -d --name redis -e "IP=0.0.0.0" -p 7000-7005:7000-7005 grokzen/redis-cluster:latest
docker run -d --name nginx --net="host" -v /home/ubuntu/nginx.conf:/etc/nginx/nginx.conf:ro nginx

# Run Server
cd /opt/signal/Signal-Server/service/target
java -jar TextSecureServer-4.97.jar accountdb migrate /home/ubuntu/config.yml
java -jar TextSecureServer-4.97.jar abusedb migrate /home/ubuntu/config.yml
java -jar TextSecureServer-4.97.jar messagedb migrate /home/ubuntu/config.yml
java -jar -Ddw.logging.level=ERROR TextSecureServer-4.97.jar server /home/ubuntu/config.yml
