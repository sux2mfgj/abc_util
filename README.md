# abc_util
a **standalone** cli tool for abc, therefore we can use **only download it**.

### Installation
```
$ curl -s https://api.github.com/repos/sux2mfgj/abc_util/releases/latest \
        | grep "browser_download_url" | cut -d : -f 2,3 \
        | tr -d \" \
        | wget -qi -
$ tar xvf abc_util_v0.1.0_for_x86_64_linux.tar.gz
$ cp ./abc_util <install path, (e.g. $HOME/local/bin)>
```

### How to use.
TBD

### License
MIT. please read the [LICENSE](./LICENSE) file.

### Special Thanks
- [clap](https://github.com/clap-rs/clap)
- [reqwest](https://github.com/seanmonstar/reqwest)
- [scraper](https://github.com/programble/scraper)
