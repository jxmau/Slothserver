1. File structure

2. Server struct with the port variable, and Route field.

```rs
Server {
    port: u8,
    routes : Vec<Route>
}
```


```rs
Route {
    name : String,
    path : String,
    response: Response,
    
}

```

```rs
Response {
    status: u8,
    cookies : Vec<Cookie>, // Create a Cookie struct for modulability
    header : HashMap<String, String>,
    body : String
}

```


```Cookie {
    name: String, 
    value: String
}
```

[ ] - Make sure the Content-Type is given.
[ ] -
[ ] -
[ ] -
[ ] -

