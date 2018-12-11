# Geocluster 🌍✨🛫

## Introduction
The Geocluster is a service that has the ability to return information about places using a REST API. Places are defined within this API as establishments, geographic locations, or prominent points of interest.

### Services provided

#### REST API

##### Places
Endpoints used to populate the database, make geospacial queries about existing Places

```
GET     /api/v1/gspots application/json                       (index_places)
GET     /api/v1/gspots?<lat>&<lng> application/json           (get_possible_places)
GET     /api/v1/gspots/populate?<lat>&<lng> application/json  (populate_by_coordinate)
GET     /api/v1/gspots/<id> application/json                  (show)
DELETE  /api/v1/gspots/<id>                                   (delete)
```

##### Visits & Lists
Endpoints used to store User visits to certain places and grouping them in user specified lists, enabling the generation of personalized routes based on previous Visits

```
POST    /api/v1/visits application/json                       (new_visit)
GET     /api/v1/visits/list/<list_id> application/json        (show_visits_by_list)
DELETE  /api/v1/visits/<id>                                   (delete)

GET     /api/v1/lists application/json                        (index_list)
POST    /api/v1/lists application/json                        (new_list)
GET     /api/v1/list/<id> application/json                    (show_list_by_id)
DELETE  /api/v1/list/<id>                                     (delete_list)
GET     /api/v1/lists/user/<user_id> application/json         (lists_by_author)
PUT     /api/v1/list/<id> application/json                    (update_list_by_id)
```

##### Suggestions
Endpoints used to suggest visits and routes to users based on previous interaccions

```
GET     /api/v1/magic?<lat>&<lng>&<user> application/json     (get_route_suggestion)
GET     /api/v1/magic/change?<lat>&<lng>&<place_id>           (change_route_suggestion)
```

### Development

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

#### Extending Current API Usual Commands
These are meant to be run inside the container

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


