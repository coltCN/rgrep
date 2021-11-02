# 练手项目- rgrep

支持以下三种使用场景

### step 1

给定一个字符串以及一个文件，打印出文件中所有包含该字符串的行

```
$ rgrep Hello a.txt
55: Hello world. This is an exmaple text
```

### step 2

允许用户提供一个正则表达式，来查找文件中所有包含该字符串的行

```
$ rgrep Hel[^\\s]+ a.txt
55: Hello world. This is an exmaple text
89: Help me! I need assistant!
```

### step 3

```
$ rgrep Hel[^\\s]+ a*.txt
a.txt
    55:1 Hello world. This is an exmaple text
    89:1 Help me! I need assistant!
    5:6  Use `Help` to get help.
abc.txt:
    100:1 Hello Tyr!
```

#### crate

- 使用 clap3 处理命令行的部分
- 使用 regex 处理正则表达式
- 使用 std::fs 或者 tokio::fs 处理文件的读取
- tokio 来并行处理。
- 使用 globset 或者 glob 来处理通配符
