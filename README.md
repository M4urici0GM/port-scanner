## Multi-threaded port scanner
### Simple port scanner made with RUST.

## :computer: Used Technologies

- [RUST](https://www.rust-lang.org/)

## :computer: Usage:

```
  port-scanner [OPTIONS] -host 192.168.0.1
```

### Multi-threading
You can pass the ```-threads``` arguments to run it with multiple threads to scale the speed of scanning, e.g:

```
 port-scanner -threads 10000 -host 192.168.0.1
```
- You might run into some issue trying to specify too much threads, since thread pool is usualy managed by the host OS.
- You can check the thread limit of Unix based OS with ```cat /proc/sys/kernel/threads-max```

### Timeouts
To set a timeout for trying to connect to a port, specify it with ```-timeout``` argument:

```
 port-scanner -threads 10000 -host 192.168.0.1 -timeout 500
```



## :page_with_curl: License

That project is under MIT License, check the [LICENSE](LICENSE.md) file to read more.

---

Done with :heart: