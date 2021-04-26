#!/usr/bin/bash
set -e

if cat /proc/version | grep microsoft; then
  CMD="cmd.exe /c"
  ORIGINAL_PWD=$(wslpath -w $(pwd))
else
  CMD=
  ORIGINAL_PWD=$(pwd)
fi

sudo rm -rf dp3t-sdk-android
$CMD git clone https://github.com/DP-3T/dp3t-sdk-android
cd dp3t-sdk-android/calibration-app
$CMD git checkout prestandard
$CMD git apply --ignore-space-change --ignore-whitespace ../../local_server.patch
$CMD docker run --rm -v "$ORIGINAL_PWD":/project mingc/android-build-box bash -c 'cd /project/dp3t-sdk-android/calibration-app; ./gradlew assembleDebug'
cp "app/build/outputs/apk/debug/app-debug.apk" "../../app_context/"
cd ../../
sudo rm -rf dp3t-sdk-android
