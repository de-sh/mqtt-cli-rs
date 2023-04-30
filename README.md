# mqtt-cli-rs
An MQTT Command Line Interface built with rumqttc

## Usage
```
# Publish: sends an MQTT publish packets to the borker with the set topic, blocks till disconnection.
$ mqtt-cli-rs pub -t hello/world -m "Hello, World!"
...

# Subscribe: prints payload of MQTT publish packets that the broker forwards, based on subscribed topic, blocks only till
$ mqtt-cli-rs sub -t hello/world
Hello, World!
...
```

> **NOTE**: You can connect to a specific broker with the `-h <host>` and `-p <port>` as illustrated in the following example publishing to a broker running at example.org on port 8883.
> ```
> $ mqtt-cli-rs -h example.org -p 8883 pub -t hello/world -m "Hello, World!"
> ```
> It can be further noted that certain fields have default values and the order of certain arguments provided to the CLI are important.
