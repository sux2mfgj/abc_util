# abc_util
A **standalone** cli tool for abc, therefore we can use **only download it**.  You can see the problem statements of the each [Atcoder beginners contests](https://atcoder.jp/contests/archive?category=5&keyword=) and test your code with sample inputs in command line.

[![dependency status](https://deps.rs/repo/github/sux2mfgj/abc_util/status.svg)](https://deps.rs/repo/github/sux2mfgj/abc_util)

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
