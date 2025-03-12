*Virtual Router Redundancy Protocol* (*VRRP*) is a protocol that allows multiple routers to work as
a single router, client hosts trying to use either one, depending on whichever currently works.

Basically, either router of those that are setup will own some IP, setup as a gateway on all
clients. If the currently working router fails, some other router will take over and have the IP
assigned to its interface. The gateway IP will stay the same on the clients, so data previously sent
to the unavailable router, will be sent to the new router.

## `keepalived`

A common way to implement VRRP is with `keepalived`. Installation is done with
`sudo apt install keepalived`.

The `/etc/keepalived` directory contains a `keepalived.conf.sample` file which you may use. We will
simply create a `keepalived.conf` file with the following parts:

1. Global variables. We will make the script run as root for ease of setup.

   ```conf
   global_defs {
       enable_script_security
       script_user root
   }
   ```

2. We will define a script we name `health_check`, which will simply check that internet is
   reachable.

   ```conf
   vrrp_script health_check {
       script "/etc/keepalived/check.sh"
       interval 4
       timeout 3
       rise 3
       fall 3
   }
   ```

   The times are in seconds. `rise` and `fall` are the number of times the check succeeds or fails
   before the IP is handed over.

   The script `/etc/keepalived/check.sh` will be a simple internet ping with ignored output:

   ```bash
   #!/bin/sh
   ping -W 2 -c 1 8.8.8.8 > /dev/null 2>&1
   ```

3. Last, we setup a VRRP instance:

   ```conf
   vrrp_instance my_cool_vrrp {
       state MASTER
       interface eth0
       virtual_router_id 1
       advert_int 1
       priority 100
       virtual_ipaddress {
           192.168.0.1/24
       }
       track_interface {
           eth0
       }
       track_script {
           health_check
       }
   }
   ```

   We setup the IP address that will be passed over `192.168.0.1/24`. The `virtual_router_id`
   should be unique for all configs that setup the same IP. The `advert_int` is the time in seconds
   of how often instances tell their health to other instances.

   The state on this instance is `MASTER`, which means that this instance will own the IP address
   by default. If the `MASTER` instance fails, the `BACKUP` instance will take over. We can send
   the configuration over to other instances via SCP and change this line.

After setup is done, restart `keepalived` in `systemd` with `sudo systemctl restart keepalived`.
