# About

The API definitions part of the [Audio Cloud platform](https://github.com/audiocloud).

Audio Cloud is a collection of services that schedule, execute and automate software supported tasks dealing with
multimedia content creation and manipulation. It aims to differentiate itself by being geodistributed and controllable
in real time.

# Directory layout (APIs)

Inside `audiocloud-api/src` you will find the following directories:

| Path           | Description                                                                                   |
|----------------|-----------------------------------------------------------------------------------------------|
| `common`       | Types shared by all APIs                                                                      |
| `api`          | Metadata for APIs                                                                             |
| `cloud`        | Cloud Service API requests and responses, REST paths                                          |
| `domain`       | Domain Service API requests and responses, REST paths, Kafka messages, WebSocket/RTC messages |
| `driver`       | Instance Driver API requests and responses, REST paths, NATS subjects                         |
| `audio_engine` | Audio Engine API requests and responses, REST paths, NATS subjects                            |
| `app`          | Apps API requests and responses, REST paths                                                   |
| `converter`    | Format converter API requests and responses, REST paths                                       |
| `waveformer`   | Waveform generator API requests and responses, REST paths                                     |

# Generated clients

