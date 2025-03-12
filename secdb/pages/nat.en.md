*Network Address Translation* (*NAT*) is a mechanism that replaces IP addresses in network packets,
so that forwarded packets do not contain information about either side of the router.

Most common NAT setup is *masquerade*, which is when the router receives a packet from some private
host, it replaces its IP by the IP that the router has, in order to make it seem like the packet
came from the router itself. This means the internet/ISP will see the packet from the device that is
connected to it, rather than from some unknown private IP address. This will in turn let private
hosts use internet.

## `nftables`

A common way to implement NAT is with `nftables`. Installation is done with
`sudo apt install nftables`, but if it is already installed, you may just need to enabled in your
`systemd` with `sudo systemctl enable --now nftables`.

After this, you can configure the `/etc/nftables.conf` file (there are equivalent commands, but it
is not as ergonomic).

Most common part of NFT is tables, which are collections of rule chains. The chains are used to
apply rules to packets as they pass through the router. We only need to setup a `nat` table with the
`postrouting` chain, which is used to modify the packets after it is determined where the packet
should go by means of routing.

```conf
table ip nat {
    chain postrouting {
        type nat hook postrouting priority 0;
        oif "enp1s0" masquerade;
    }
}
```

### Troubleshooting

- Ensure [IP forwarding](network#ip-forwarding) is on.
- Ensure `nftables` service is enabled.
