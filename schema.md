# Config Schema

## Cloud-init

Cloud init may be used to provides two files `meta-data` and `user-data` to the guest to initialise it 
(https://cloudinit.readthedocs.io/en/latest/)

KVM Compose can generate both of these automatically or you can override and choose your own files

```yaml
    cloud_init:
      user_data: ./user_data_1
      meta_data: ./meta_data_1
```

When auto generated:

Meta data will contain:

- Hostname

User data will contain:

- SSH Public Key

Generating the user data automatically requires some information about the guest operating system,
when using a CloudImage this is inferred automatically. Otherwise you must specify the operating system for user-data to
be generated:

```yaml
    os: cirros
```
