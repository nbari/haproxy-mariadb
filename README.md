# haproxy-mariadb
HAproxy + MariaDB disconnect issue

Run using:

    make

Test using:

    mycli -uroot --pass secret

On the logs when disconnection you will see something like:

     [Warning] Aborted connection 14 to db: 'unconnected' user: 'root' host: '172.18.0.2' (Got an error reading communication packets)
