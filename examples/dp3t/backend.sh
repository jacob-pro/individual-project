set -e

sudo apt update
sudo apt install -y git make maven
mkdir -p /opt/dp3t
cd /opt/dp3t
git clone https://github.com/DP-3T/dp3t-sdk-backend.git
cd dp3t-sdk-backend/dpppt-backend-sdk
# Use pre-standard server
git checkout v1.1.2
git apply --ignore-space-change --ignore-whitespace /etc/nocloud/context/no_bintray.patch
mvn install -DskipTests

properties="
ws.origin.country=uk
spring.profiles.active=dev
ws.ecdsa.credentials.privateKey=LS0tLS1CRUdJTiBQUklWQVRFIEtFWS0tLS0tCk1JR1RBZ0VBTUJNR0J5cUdTTTQ5QWdFR0NDcUdTTTQ5QXdFSEJIa3dkd0lCQVFRZ0IxQ0E4QmhkVENhQzkvMWoKbTZVcFNhQXlTL01Kc3ZhdmJIQldWaDc5SCtHZ0NnWUlLb1pJemowREFRZWhSQU5DQUFSNmg5UXVhTi9VelZ4WgphUDBFV2h3Zm8yRUs5aUNUbWQyWWhrQ0NKTzY2QjM3VWxGdVpseGxnS2w2cENZTGNVK1pzMDZ3eERYOUI0dFcwCm14WG1nc09CCi0tLS0tRU5EIFBSSVZBVEUgS0VZLS0tLS0K
ws.ecdsa.credentials.publicKey=LS0tLS1CRUdJTiBQVUJMSUMgS0VZLS0tLS0KTUZrd0V3WUhLb1pJemowQ0FRWUlLb1pJemowREFRY0RRZ0FFZW9mVUxtamYxTTFjV1dqOUJGb2NINk5oQ3ZZZwprNW5kbUlaQWdpVHV1Z2QrMUpSYm1aY1pZQ3BlcVFtQzNGUG1iTk9zTVExL1FlTFZ0SnNWNW9MRGdRPT0KLS0tLS1FTkQgUFVCTElDIEtFWS0tLS0tCg==
"
echo "$properties" > application.properties

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

# Anbox requires a static IP
netplan="
network:
  version: 2
  renderer: networkd
  ethernets:
    enp0s3:
      addresses:
        - 10.3.10.99/24
      gateway4: 10.3.10.254
      nameservers:
          addresses: [10.3.10.254]
"
echo "$netplan" > /etc/netplan/99_config.yaml
echo "network: {config: disabled}" > /etc/cloud/cloud.cfg.d/99-disable-network-config.cfg
netplan apply

echo "DONE!"
