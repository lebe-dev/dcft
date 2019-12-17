# Docker Compose File Tool

Manage docker-compose.yml file.

## Features

1. Replace image and tag for service

## Usage

### Replace image and tag for service

dcft -r <service> <image> <tag>

Example:

```
dcft -ri backend registry.domain.com/app-frontend 2.1.7
```

Set image and tag `registry.domain.com/app-frontend:2.1.7` for service `backend`