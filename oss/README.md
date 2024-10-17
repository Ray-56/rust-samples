# Upload Files to OSS

## How to Use

Configure the following environment variables in the .env or .env.local file:

```shell
OSS_ACCESS_KEY_ID=xxx
OSS_ACCESS_KEY_SECRET=xxx
OSS_BUCKET=xxx
OSS_ENDPOINT=xxx
```

```shell
> ./oss -i foo -d example/foo --html-last
```

## Parameter Description

```sh
> ./oss -h
aliyun oss actions

Usage: oss [OPTIONS] --dest-dir <DESTINATION_DIRECTORY> <--input-root <INPUT_ROOT>|--input-pattern <INPUT_PATTERN>>

Options:
  -i, --input-root <INPUT_ROOT>           The input directory
  -p, --input-pattern <INPUT_PATTERN>     The input pattern
  -d, --dest-dir <DESTINATION_DIRECTORY>  The destination directory
      --html-last                         The html file is last upload
  -h, --help                              Print help
  -V, --version                           Print version
```
