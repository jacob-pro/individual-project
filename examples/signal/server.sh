# Guide to setup Signal Server
# Must be on an Ubuntu AWS Instance
# AWS Instance must have an IAM role
# Role must access a deployed AWS AppConfig with Environment

# Copy config file
scp .\config.yml ubuntu@${SIGNAL_IP}:/home/ubuntu/

# Disable lines in service/src/main/java/org/whispersystems/textsecuregcm/WhisperServerService.java
#   // environment.lifecycle().manage(accountDatabaseCrawler);
#   // environment.lifecycle().manage(remoteConfigsManager);

# Download and build server
sudo su
apt update
apt install -y git maven
mkdir -p /opt/signal
cd /opt/signal
git clone https://github.com/signalapp/Signal-Server.git
cd Signal-Server
git checkout 1999bd2bcbf88162325f446119e8f10e0291fdb5
mvn -DskipTests package

# Install Docker + Dependencies
https://docs.docker.com/engine/install/ubuntu/
cd /opt/signal
docker run -d --name postgres -e "POSTGRES_PASSWORD=postgres" -e "POSTGRES_DB=signal" -p 5432:5432 postgres:11
docker run -d --name redis -e "IP=0.0.0.0" -p 7000-7005:7000-7005 grokzen/redis-cluster:latest

# Run Server
cd /opt/signal/Signal-Server/service/target
java -jar -Ddw.logging.level=ERROR TextSecureServer-5.80.jar server /home/ubuntu/config.yml
