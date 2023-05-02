# Build

```
$ docker build -t <image_name> -f ./Dockerfile ./<challenge_folder>
```

# Run

```
$ docker run -v $(pwd)/store:/main/store -t <image_name> test [maelstrom args...]
```

# Example

```
$ docker build -t echo -f ./Dockerfile ./echo
(...)
$ docker run -v $(pwd)/store:/main/store -t echo test -w echo --bin ./echo --node-count 1 --time-limit 10
(...)

Everything looks good! ヽ(‘ー`)ノ
```

Maelstrom's output logs and files can be found in ./store.