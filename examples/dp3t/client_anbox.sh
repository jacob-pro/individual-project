set -e

# Xorg
apt update
apt install -y xorg adb
apt install -y --no-install-recommends openbox

# Anbox Kernel modules: https://docs.anbox.io/userguide/install_kernel_modules.html
add-apt-repository ppa:morphis/anbox-support
apt update
apt install -y linux-headers-generic anbox-modules-dkms
modprobe ashmem_linux
modprobe binder_linux
# Anbox
snap install --devmode --beta anbox

cd /
wget https://github.com/Debyzulkarnain/anbox-bridge/raw/master/anbox-bridge.sh
chmod +x anbox-bridge.sh

echo "DONE! - Rebooting"
reboot

# Usage - https://docs.anbox.io/userguide/install_apps.html
# startx
# adb devices
# sudo /anbox-bridge.sh start
# anbox.appmgr
# adb install /etc/nocloud/context/app-debug.apk
