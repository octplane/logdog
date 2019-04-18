# Log dog

Send any text to datadog log quickly.

# Get it!

- [there](https://github.com/octplane/logdog/releases)

# Usage

## Configuration

- `DD_API_KEY` in your env. **mandatory**
- `DD_SITE` to target another sire. Defaults to datadoghq.com. Logdog will use `intake.logs.${DD_SITE}:10516`
- `DD_URL`: your base datadog url. Defaults to https://app.datadoghq.com/

## Run

```shell
> logdog < whatever.log
https://app.datadogh.com/logs? \
   cols=event& \
   index=main& \
   live=true& \
   query=source%3Alog-pipe+service%3Acli-client+session%3Asess[SOMERANDOMNUMBER]& \
   sort=desc& \
   stream_sort=desc
```

Open the returned url:

![](logdog.png)

# Limitations

- generated URL is very stupid and use hardcoded settings everywhere
- not very well tested (help me with that)
- only macOS version for now

# Build it

```shell
cargo build --release
```

