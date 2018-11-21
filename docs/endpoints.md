# Endpoints REST API

## /api/v1/lists
### Retrieve all lists

GET
RESPONSE
{
    "result": [
        {
            "id": 5,
            "list_name": "Restaurantes Aveiro",
            "user_id": 1
        }
    ],
    "status": 200
}

## /api/v1/lists
### Create new list

POST
_header_
Content-Type: application/json
_body_
{
    "user_id": 1,
    "list_name": "Restaurantes Aveiro"
}
RESPONSE
{
    "result": {
        "id": 5,
        "list_name": "Restaurantes Aveiro",
        "user_id": 1
    },
    "status": true
}


## /api/v1/list/<id>
### Retrieve specific lists

GET
RESPONSE

## /api/v1/list/id
### Update specific lists
(same as before )
PUT
RESPONSE

## /api/v1/list/id
### Delete specific lists
(same as before )
DELETE
RESPONSE

## /api/v1/list/user/<user_id>
### Retrieve lists by user
(same as before )
GET
RESPONSE

----------------------------------------------------

## /api/v1/visits

POST
_header_
Content-Type: application/json
_body_
{
    "list_id": 1,
    "internal_id_place": "Restaurantes Aveiro"
    "review": "Restaurantes Aveiro"
}
RESPONSE
{
    "result": {
        "id": 5,
        "list_name": "Restaurantes Aveiro",
        "user_id": 1
    },
    "status": true
}
