# Docker Setup

## Prerequisites

- [Docker](https://docs.docker.com/engine/)
- [Docker Compose](https://docs.docker.com/compose/install/)

## Setup

Build the userbot image first (Note that this command needs to be run after every update)

`docker-compose build`

The initial setup requires a `userbot.session` file. If you already have it, skip this step  

```shell
touch userbot.session
docker-compose run userbot
```

Subsequent runs can use daemon mode

`docker-compose up -d`
