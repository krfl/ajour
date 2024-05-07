# Sidequest

WIP CLI tool for keeping track all the sidequests at work

## Add

Add a new entry

```
sq add "customer support requested help with figuring out what went wrong"
```

Add also works without it's keyword, but will still react to reserved words such as "add", "list" and "export"

```
sq customer support requested help with figuring out what went wrong
```

## List

List all stored entries

```
sq list
```

Limit what's being listed by using filters: --from/-f and --to/-t

```
sq list --from 2023-12-24 --to 2023-12-31
```
