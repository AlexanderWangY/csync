# csync
A content-addressable file synchronization engine written in Rust. csync takes a lot of inspiration from rsync, Git, and a variety of file servers.
`csync` utilizes chunk-based data dedeuplication to efficiently store and update files.

> This is **NOWHERE** close to being finished. PRs and help welcome, but please **DO NOT** actually use this right now.


## Overview
`csync` requires you to have a server for storing files and the ability to expose it to your network (local if only syncing within local network). It utilizes a Git like remote to resolve
where that specific file should be synced from. Consequently, this means that different files can have different hosts which means you can distribute your file storage across
many different file servers.

Another bonus of `csync` is that it utilizes CAS (content-addressable storage) to effeciently dedeuplicate chunks that have already been stored and/or downloaded. This means
it reduces network and disk I/O strain when dealing with large files. Woohoo!

## Structure
You may be asking, where do we actually store mappings of files, remote locations, and data storage (server side). We store everything in the following locations

- `~/.csync/data` - Chunk/blob storage, used server/daemon side.
- `~/.csync/manifests` - JSON mapping file versions and chunk lists.
- `~/.csync/config.toml` - Configurations to modify the default behavior of `csync`

## Target API/Usage

We have two binaries available: `csyncd` for the file server daemon and `csync` for our client CLI.
Again, these commands and usages are *planned* to be built. Not done yet.

### Daemon

Start server (Not done)
```
  csyncd start --port 8080 # Our default port is 80085
```


### CLI

Sync/pull a file from a remote endpoint: (Not done)
```
  csync pull <remote_url>:<file_id> ./file.txt
```


Initialize a file to track a remote source: (Not done)
```
  csync set-remote ./my_file.txt <remote_url>:<file_id>
```


Push changes (must have set file remote!): (Not done)
```
  csync push ./my_file
```


Check remote & local status: (Not done)
```
  csync status ./my_file
```


Diff local file with remote: (Not done)
```
  csync diff ./my_file
```
