# Slothserver

This is the repository of the Sloth module for generating a mock Server.



## Library used:

* serde, features: derive
* serde_json
* log
* [Arkos](https://github.com/jxmau/arkos), branch v.1.2


## Example

```json
{
    "port": 8080,
    "routes": [  
        {
            "name": "Route",
            "path": "/",
            "method": "GET",
            "response" : {
                "status": 200,
                "cookies": [],
                "headers": { 
                    "Name":"Value"
                },
                "body":{"Hello": "World!"}
            }
        }
    ]
}

```