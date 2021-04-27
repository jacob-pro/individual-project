set -e
apt update
apt install -y xorg adb
apt install -y --no-install-recommends openbox

# https://linuxhint.com/setup-android-emulator-without-installing-android-studio-in-linux/
mkdir -p /android/cmdline-tools
cd /android/cmdline-tools
wget https://dl.google.com/android/repository/commandlinetools-linux-6858069_latest.zip
unzip commandlinetools-linux-6858069_latest.zip
mv cmdline-tools/ tools/
cd tools/bin
./sdkmanager
echo y | ./sdkmanager platform-tools emulator
echo y | ./sdkmanager "platforms;android-30" "system-images;android-30;google_apis;x86" "build-tools;30.0.2"
echo no | ./avdmanager create avd -n "avd_30" -k "system-images;android-30;google_apis;x86"

echo "DONE! - Rebooting"
reboot

# Usage:
# startx
# cd /android/emulator
# export QTWEBENGINE_DISABLE_SANDBOX=1
# sudo ./emulator -avd "avd_30"
# adb devices
# adb install -t /etc/nocloud/context/app-debug.apk
# adb -e logcat org.dppt.demo

