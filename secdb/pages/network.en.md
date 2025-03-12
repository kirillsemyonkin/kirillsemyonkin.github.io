In most linux systems networking is handled by the `ip` command. It is usually included, but in
places like Docker it is not. In this case you can install it with `sudo apt install iproute2`.

## Interfaces

Linux networking is done via interfaces. Each interface represents a network device inside your
machine (e.g. a physical port where you would plug an ethernet cable). Each such device has a unique
number associated with it called Medium Access Control address (MAC address), which allows devices
and you to know what interface represents which device.

To see all interfaces:

```bash
$ ip link  # or ip l
1: lo: <LOOPBACK,UP,LOWER_UP> mtu 65536 qdisc noqueue state UNKNOWN group default qlen 1000
    link/loopback 00:00:00:00:00:00 brd 00:00:00:00:00:00
2: eth0: <BROADCAST,MULTICAST,UP,LOWER_UP> mtu 1434 qdisc mq state UNKNOWN group default qlen 1000
    link/ether 00:15:5d:43:b5:9e brd ff:ff:ff:ff:ff:ff
```

Currently we see two interfaces: `lo` (loopback) and `eth0` (ethernet). First one is your
`localhost` address, so talking to it means interacting with the host itself. The second interface
is your *external* interface, that allows you to connect to the internet or to other local machines.
It can have numerous names depending on where the machine is running (for example, `enp0s1` or
`enp1s0` in a VM).

## IP setup

Each device can also have an IP address, which allows not only directly connected neighbors to talk
to each other, but also allows communication across chains of multiple devices. In this example we
will be looking at IPv4, which looks like the following:

```plain
# Just IP
244.178.44.111
--- octet, as it is a number from 0 to 255 (8 bits)

# IP with Mask
244.178.44.111/24
               -- mask, bit amount from 0 to 32
```

The mask allows to specify which part of the address represents *network*, and which is a *host* in
that network. Two devices in the same network should be neighbors, e.g. connected either with a wire
or a switch, but only if they are not a part of two different private networks (e.g. both Wi-Fi
networks located across the ocean can have same local network addresses, but they will not be able
to communicate with each other without extra work).

To visualize the network, look at the following:

```plain
# in binary, octets are 8 bits
11110100.10110010.00101100.01101111
# mask can be represented with bits as well
11111111.11111111.11111111.00000000
# 24 bits is 24 ones on the left side, and 8 zeros represents all possible hosts in the network
# same mask in decimal
255.255.255.0
```

The only exceptions for host addresses:

- First address from the network, e.g. `0` - represents the network itself (e.g. specifying
    `192.168.0.0` somewhere means "I accept any connection from network", similarly `0.0.0.0` means
    "I accept any connection")
- Last address from the network, e.g. `255` - reserved for broadcast (e.g. `192.168.1.255` means all
    devices on the network should accept messages addressed to it).

To attach an IP address to an interface, as well as show it, you can use `ip addr` or `ip a`:

```bash
$ ip a add 192.168.0.178/24 dev eth0
$ ip a
...
2: eth0: <BROADCAST,MULTICAST,UP,LOWER_UP> mtu 1434 qdisc mq state UP group default qlen 1000
    link/ether 00:15:5d:43:b5:9e brd ff:ff:ff:ff:ff:ff
    inet 192.168.0.178/24 brd 172.19.223.255 scope global eth0
       valid_lft forever preferred_lft forever
```

This will specify an address `192.168.0.178` with mask `24` on the `eth0` interface. However, this
will not be saved and will be lost on reboot. To alleviate this, you should either make a startup
script, or use the `/etc/network/interfaces` file.

## The `interfaces` file

Simplest setup for an interface looks like the following:

```conf
auto eth0
iface eth0 inet static
    address 192.168.0.178
    netmask 255.255.255.0
```

- `auto eth0` means the interface should start automatically.
- `iface eth0 inet static` means the interface `eth0` is using static IPv4 setup (`inet6` is IPv6).
- `address` is the IP address assigned to the interface.
- `netmask` is the mask for the network.

Once you are done with the changes to that file, restart the `networking` service. In `systemd`:

```bash
sudo systemctl restart networking
```

## Routes

Routes allow to specify the way to communicate across networks. The idea is simple: the host can
simply know whom to talk to in case the IP address is from a different network that is not the
direct neighbor of itself.

A route for all networks the device is present in is created automatically:

```bash
$ ip route  # or `ip r`
192.168.0.0/20 dev eth0 proto kernel scope link
```

It contains 2 parts:

- `192.168.0.0/20` - the network that we want to reach.
- `dev eth0` - the interface, from which this route is used.

This means all addresses from `192.168.0.0/20` can be reached from `eth0` interface.

Manual addition of routes with the `ip` command is following:

```bash
ip route add 244.178.44.0/24 dev eth0 via 237.84.2.178
```

This route means "to communicate with network `244.178.44.0/24`, use interface `eth0` and send the
data to `237.84.2.178`". Such a route needs to be present on both sides, else the other side may not
know where to send the reply back.

A more useful route is named `default`. It is the route that is used by default if no other route
is matched otherwise, which allows for communicating with any IPs that are not directly in the
neighboring networks. It is also named *gateway*, since to specify it we need to tell what other
device (e.g. router) should forward the traffic to external/unknown networks. It is useful (and
usually makes sense only) to setup this route for the interface that should be connected to the
internet.

To automatically setup this route, we can add this line to our `interfaces` file:

```conf
auto eth0
iface eth0 inet static
    address 192.168.0.178/24 # you can also specify `netmask` with `/<mask>`
    gateway 192.168.0.1
```

The resulting route looks like this:

```bash
$ ip r
default via 192.168.0.1 dev eth0 proto kernel
```

This means to communicate with all networks that are not directly reachable, you can send the data
to the router `192.168.0.1`.

## IP forwarding

If your host is a router, you want to enable *IP forwarding*. This means that the host will forward
incoming data to its connected hosts (e.g. router's default route), as opposed to the default
behavior of throwing out data not addressed to that router in particular.

To enable this, add the following line to the `/etc/sysctl.conf` file:

```conf
net.ipv4.ip_forward = 1
```

After this, see it applied with the following command:

```bash
$ sudo sysctl -p
net.ipv4.ip_forward = 1
```
