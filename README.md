# Ajour

WIP CLI tool for keeping track all the sidequests at work

## Add

Add a new entry

```
ajour add "customer support requested help with figuring out what went wrong"
```

Add also works without it's keyword, but will still react to reserved words such as "add", "list" and "export"

```
ajour customer support requested help with figuring out what went wrong
```

## List

List all stored entries

```
ajour list
```

Limit what's being listed by using filters: --from/-f and --to/-t

```
ajour list --from 2023-12-24 --to 2023-12-31
```
