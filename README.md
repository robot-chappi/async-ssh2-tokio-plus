# My Fork of async-ssh2-tokio

This project is a fork of [async-ssh2-tokio](https://github.com/Miyoshi-Ryota/async-ssh2-tokio).

## Modifications
- Added `download_file` functionality for SFTP sessions.
- Updated `russh-sftp` version to `2.0.6`.
- Other minor adjustments to logging and error handling.

## Warning
My test test_download_file.rs it is not executed correctly automatically, it was created to dynamically check the functionality of my new download_file function!

## License
This project is licensed under the [Apache License 2.0](LICENSE), as originally stated by the authors of async-ssh2-tokio.
