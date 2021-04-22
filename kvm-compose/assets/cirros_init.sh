
mkdir /home/cirros/.ssh
cirros-query get public_ssh_key > /home/cirros/.ssh/authorized_keys
chmod 600 /home/cirros/.ssh/authorized_keys
chown cirros /home/cirros/.ssh/authorized_keys

mkdir -p /etc/nocloud
cirros-query get run_script > /etc/nocloud/run_script.sh
chmod +x /etc/nocloud/run_script.sh
sh /etc/nocloud/run_script.sh &> /etc/nocloud/run_script_log.txt
