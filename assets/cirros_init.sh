
mkdir /home/cirros/.ssh
cirros-query get public_ssh_key > /home/cirros/.ssh/authorized_keys
chmod 600 /home/cirros/.ssh/authorized_keys
chown cirros /home/cirros/.ssh/authorized_keys

cirros-query get run_script > run_script.sh
chmod +x run_script.sh
sh run_script.sh
