Secret-santa-as-a-service
========================

Secret-santa-as-a-service is a HTTP service designed to match user in a secret
santa scheme. 

Usage
-----

`./bin/ssaas key`

This start the service with the admin key

`curl -X POST 127.0.0.1/subscribe?name=John%20Doe&password=XXX`

This will create your subscription and return a link to your user

`curl -X POST 127.0.0.1/start?key=KEY`

This will pair all users

`curl GET 127.0.0.1/user/John%20Doe`

This will remind you of your match



