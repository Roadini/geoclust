# Geo Clust 🌍

## A web based service with stuff about geolocation


##### Development

Build image from docker file

```
docker build . -t geoclust
```

It is assumed that the composer already created the docker network, if not:
```
docker network create roadiniserver_proxynet
```

Finnaly
```
docker-compose up
```


###### Extending Current API Usual Commands
Theser are meant to be run inside the container

- Generate migration

example with visits
```
diesel migration generate create_visits
```

After writing new migration
- Aply
```
diesel migration run
```

