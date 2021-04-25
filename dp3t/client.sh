set -e
# Anbox Kernel modules: https://docs.anbox.io/userguide/install_kernel_modules.html
add-apt-repository ppa:morphis/anbox-support
apt update
apt install -y linux-headers-generic anbox-modules-dkms
modprobe ashmem_linux
modprobe binder_linux
# Anbox
snap install --devmode --beta anbox
