# DP3T

This is an example usage of the project - testing the DP3T COVID app service.

https://github.com/DP-3T/dp3t-sdk-backend

https://github.com/DP-3T/dp3t-sdk-android

There are two different Android VMs defined:

Anbox:
 - Uses Linux containers to run Android directly - better performance
 - Android apps use Google DNS

Android SDK / Emulator:
 - Requires KVM nested virtualization (must be enabled on host) - slower performance
 - Apps use the same name resolution as the VM
 - More features: sensor simulation, GPS etc.
