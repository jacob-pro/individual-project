set -e
sudo apt update
sudo apt install -y git make maven
mkdir -p /opt/dp3t
cd /opt/dp3t
git clone https://github.com/DP-3T/dp3t-sdk-backend.git
cd dp3t-sdk-backend/dpppt-backend-sdk
mvn install -DskipTests
echo "ws.origin.country=uk" >> application.properties
echo "spring.profiles.active=dev" >> application.properties
# java -jar dpppt-backend-sdk-ws/target/dpppt-backend-sdk-ws.jar
