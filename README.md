# haproxy-mariadb
HAproxy + MariaDB disconnect issue

Run using:

    make

Test using (password is `secret`) :

    mycli -h 127.0.0.1 --port 3306 -uroot --pass secret

or

    mysql -h 127.0.0.1 --port 3306 -uroot -p



On the logs when disconnection you will see something like:

     [Warning] Aborted connection 14 to db: 'unconnected' user: 'root' host: '172.18.0.2' (Got an error reading communication packets)

## Client issue

The problem seems to be on the client-side, in this example when using `mycli`
version `1.25.0` the error is present also when logging, but if using the mariadb
`mysql` client `mysql Ver 15.1 Distrib 10.8.3` there is no error present.
