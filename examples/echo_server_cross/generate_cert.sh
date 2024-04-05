set -e

# Creates a new local CA
mkcert -install

# Change the domain name if you need something different
mkcert localhost 127.0.0.1 ::1
openssl x509 -in localhost+5.pem -outform der -out localhost.der
openssl rsa -outform der -in localhost+5-key.pem -out localhost_key.der

# Prints location of CA root. Do NOT give the `rootCA-key.pem` file to anyone.
mkcert -CAROOT

read rm 
#"Press enter to continue"
