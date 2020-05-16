# create root ca. 10 years valid, no password (nodes)
# https://gist.github.com/fntlnz/cf14feb5a46b2eda428e000157447309


openssl genrsa -des3 -out ruspi_root_ca.key 4096
openssl req -x509 -new -nodes -key ruspi_root_ca.key -sha256 -days 3650 -subj "/C=CH/ST=BE/O=ruspi.dev/CN=ruspi.dev" -out ruspi_root_ca.crt

mkdir test1-ruspi-dev
openssl genrsa -out test1-ruspi-dev/test1.ruspi.dev.key 4096
openssl req -new -key test1-ruspi-dev/test1.ruspi.dev.key -subj "/C=CH/ST=BE/O=ruspi.dev/CN=test1.ruspi.dev" -out test1-ruspi-dev/test1.ruspi.dev.csr
openssl req -in test1-ruspi-dev/test1.ruspi.dev.csr -noout -text
openssl x509 -req -in test1-ruspi-dev/test1.ruspi.dev.csr -CA ruspi_root_ca.crt -CAkey ruspi_root_ca.key -CAcreateserial -out test1-ruspi-dev/test1.ruspi.dev.crt -days 3650 -sha256
openssl x509 -in test1-ruspi-dev/test1.ruspi.dev.crt -text -noout

mkdir test2-ruspi-dev
openssl genrsa -out test2-ruspi-dev/test2.ruspi.dev.key 4096
openssl req -new -key test2-ruspi-dev/test2.ruspi.dev.key -subj "/C=CH/ST=BE/O=ruspi.dev/CN=test2.ruspi.dev" -out test2-ruspi-dev/test2.ruspi.dev.csr
openssl req -in test2-ruspi-dev/test2.ruspi.dev.csr -noout -text
openssl x509 -req -in test2-ruspi-dev/test2.ruspi.dev.csr -CA ruspi_root_ca.crt -CAkey ruspi_root_ca.key -CAcreateserial -out test2-ruspi-dev/test2.ruspi.dev.crt -days 3650 -sha256
openssl x509 -in test2-ruspi-dev/test2.ruspi.dev.crt -text -noout

mkdir test3-ruspi-dev
openssl genrsa -out test3-ruspi-dev/test3.ruspi.dev.key 4096
openssl req -new -key test3-ruspi-dev/test3.ruspi.dev.key -subj "/C=CH/ST=BE/O=ruspi.dev/CN=test3.ruspi.dev" -out test3-ruspi-dev/test3.ruspi.dev.csr
openssl req -in test3-ruspi-dev/test3.ruspi.dev.csr -noout -text
openssl x509 -req -in test3-ruspi-dev/test3.ruspi.dev.csr -CA ruspi_root_ca.crt -CAkey ruspi_root_ca.key -CAcreateserial -out test3-ruspi-dev/test3.ruspi.dev.crt -days 3650 -sha256
openssl x509 -in test3-ruspi-dev/test3.ruspi.dev.crt -text -noout
