# server

## Installation

- Create a ``docker-compose.yml`` with the following content:
```
version: '3.7'
services:
  susmanager-server:
    container_name: susmanager-server
    image: optinux/susmanager:latest
    restart: unless-stopped
    ports:
      - 6969:6969
```
- Now spin up the container by executing ``sudo docker-compose -up -d``
- The service will now be accessible on port ``6969`` by default, if desired change the port settings like shown below:
```
 ports:
      - xxxx:6969
```

## Nginx Reverse Proxy Config
```
server {
    listen 80;
    
    server_name subdomain.example.com;
        location / {
           proxy_pass http://x.x.x.x:6969/;
    }
}
```
- Change the variables to fit your configuration
- Use Certbot to enable HTTPS (``sudo certbot --nginx`` and then select your domain)