# kvm-compose

## About

A command line tool for automatically building a virtual environment for use in testing privacy technologies.

Usage: run `up` or `down` in a directory with `kvm-compose.yaml` file 
describing the configuration you wish to create / destroy.

## Dependencies

Compile-time
```
sudo apt install libvirt-dev libssl-dev
```

Runtime:
```
sudo apt install qemu-kvm libvirt-daemon-system libvirt-clients openvswitch-switch
```
