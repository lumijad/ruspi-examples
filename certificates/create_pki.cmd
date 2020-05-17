REM ********************************************************************************************
REm Create directory rsa
REM ********************************************************************************************
rmdir /S /Q rsa
mkdir rsa


REM ********************************************************************************************
REm Create ruspi ca. 10 years valid. No password
REM ********************************************************************************************
openssl req -nodes -x509 -days 3650 -newkey rsa:4096 -keyout rsa/ruspi-ca.key -out rsa/ruspi-ca.crt -sha256^
          -subj "/CN=ruspi RSA CA"

REM ********************************************************************************************
REM Create intermediate ca. 10 years valid. No password
REM ********************************************************************************************

openssl req -nodes -newkey rsa:4096 -keyout rsa/ruspi-inter.key -out rsa/ruspi-inter.req -sha256^
          -subj "/CN=ruspi RSA intermediate"

openssl x509 -req -in rsa/ruspi-inter.req -out rsa/ruspi-inter.crt -CA rsa/ruspi-ca.crt -CAkey rsa/ruspi-ca.key^
            -CAcreateserial -sha256 -days 3650 -extensions ruspi_inter -extfile openssl.cnf

type rsa\ruspi-inter.crt rsa\ruspi-ca.crt > rsa\ruspi-ca-chain.crt

REM ********************************************************************************************
REM Create server certificate test1.ruspi.dev. 10 years valid. No password
REM ********************************************************************************************

openssl req -nodes -newkey rsa:4096 -keyout rsa/test1.ruspi.dev.key -out rsa/test1.ruspi.dev.req -sha256^
          -subj "/CN=test1.ruspi.dev"

openssl x509 -req -in rsa/test1.ruspi.dev.req -out rsa/test1.ruspi.dev.crt -CA rsa/ruspi-inter.crt^
            -CAkey rsa/ruspi-inter.key -sha256 -days 3650 -CAcreateserial^
            -extensions ruspi_test1 -extfile openssl.cnf

type rsa\test1.ruspi.dev.crt rsa\ruspi-ca-chain.crt > rsa\test1.ruspi.dev-fullchain.crt

REM ********************************************************************************************
REM Create server certificate test2.ruspi.dev. 10 years valid. No password
REM ********************************************************************************************

openssl req -nodes -newkey rsa:4096 -keyout rsa/test2.ruspi.dev.key -out rsa/test2.ruspi.dev.req -sha256^
          -subj "/CN=test2.ruspi.dev"

openssl x509 -req -in rsa/test2.ruspi.dev.req -out rsa/test2.ruspi.dev.crt -CA rsa/ruspi-inter.crt^
            -CAkey rsa/ruspi-inter.key -sha256 -days 3650^
            -extensions ruspi_test2 -extfile openssl.cnf

type rsa\test2.ruspi.dev.crt rsa\ruspi-ca-chain.crt > rsa\test2.ruspi.dev-fullchain.crt

REM ********************************************************************************************
REM Create server certificate test3.ruspi.dev. 10 years valid. No password
REM ********************************************************************************************

openssl req -nodes -newkey rsa:4096 -keyout rsa/test3.ruspi.dev.key -out rsa/test3.ruspi.dev.req -sha256^
          -subj "/CN=test3.ruspi.dev"

openssl x509 -req -in rsa/test3.ruspi.dev.req -out rsa/test3.ruspi.dev.crt -CA rsa/ruspi-inter.crt^
            -CAkey rsa/ruspi-inter.key -sha256 -days 3650^
            -extensions ruspi_test3 -extfile openssl.cnf

type rsa\test3.ruspi.dev.crt rsa\ruspi-ca-chain.crt > rsa\test3.ruspi.dev-fullchain.crt