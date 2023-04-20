# Proxies Rotator

Shuffles your socks - rotating proxy frontend server.

    # start proxy server
    proxies_rotator -v -B 127.0.0.1:1337 -L ./pr0xies.txt
    # add new proxies to list
    echo 127.0.0.1:9050 | anewer ./pr0xies.txt
    # reload proxy list
    killall -HUP proxies_rotator
    # send a request through a random proxy from list
    curl -vx socks5h://127.0.0.1:1337 https://icanhazip.com/

## List proxies format

    # provide socks5 proxies in a list like this
    # everything starting with a `#` is ignored as comment
    # with auth
    192.0.2.1:1337|username:password
    192.0.2.2:1337|username:password
    # no auth
    192.0.2.3:1337

    # empty lines are simply ignored

    # ipv6 proxies can be added like this
    [2001:0DB8::12:34]:1337
    [2001:0DB8::56:78]:1337
