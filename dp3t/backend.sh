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

service="[Unit]
Description=dp3t backend
[Service]
ExecStart=/usr/bin/java -jar dpppt-backend-sdk-ws/target/dpppt-backend-sdk-ws.jar
WorkingDirectory=/opt/dp3t/dp3t-sdk-backend/dpppt-backend-sdk
Type=simple
Restart=on-failure
RestartSec=10
[Install]
WantedBy=multi-user.target
"
echo "$service" > /etc/systemd/system/dp3t.service
systemctl daemon-reload
systemctl enable dp3t.service
systemctl start dp3t.service

