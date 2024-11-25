This code snippet demonstrates how to parse a JSON file storing where the keys and values's line number.

This example shows the line number of the content of object which has `object_key` as its key.

```sh
$ cat demo.json
```

```sh
{
    "null": null,
    "string": "string",
    "number": 123,
    "array": [1, 2, 3],
    "object_key": { 
      "name": "john"
    },
    "value": "asdf"
}

```

```sh
$ cargo run
```

```sh
key: name, line_index: 7
value: john
```
