# Build

```
$ docker build -t <image_name> -f ./Dockerfile ./<challenge_folder>
```

# Run

```
$ docker run -t <image_name> test [maelstrom args...]
```

# Example

```
$ docker build -t echo -f ./Dockerfile ./echo
(...)
$ docker run -t echo test -w echo --bin ./echo --node-count 1 --time-limit 10
(...)

Everything looks good! ヽ(‘ー`)ノ
```