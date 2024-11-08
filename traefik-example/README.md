# Traefik Example

## Description
This Traefik example is added to reverse engineer some of the features commonly provided by a real reverse proxy.

## Setup
- Ensure your Docker (Docker Daemon) is running.
- Run ```docker-compose -f traefik.yml up -d``` from ```./traefik-example``` directory.
- Access Traefik dashboard at ```http://localhost/dashboard/``` (don't forget the slash at the end of the url)
  - username: ```admin```
  - password: ```admin```
- For more details, see the official Traefik documentation at ```https://doc.traefik.io/traefik/```