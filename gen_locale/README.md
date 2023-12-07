# gen_locale

从 Excel 文件中生成多语言`.ts`文件

## TODO

- [] 内容换行符问题，目前使用字符串模板
- [x] excel文件 sheet name，考虑加入参数指定解析 sheet name; 已加入参数 -s --sheet
- [x] sheet name 不存在时，提示错误
- [] 相同id提示、错误提示等

## How to use

```bash
$ cargo run -- -i file.excel
$ cargo run -- -i file.excel -d dist
```

```bash
$ ./gen_locale -i file.xlsx -d aa
The target files is generated.

$ ls
aa       file.xlsx  gen_locale

$ ls aa
en-US.ts zh-CN.ts

$ cat aa/en-US.ts
export default {
        "bot.dashboard": "Bot Dashboard",
        "bot.dashboard.hunter.time": "Time Range",
        "bot.dashboard.hunter.time.ranges.lastWeek": "Last Week",
        "bot.dashboard.hunter.time.ranges.lastTwoWeeks": "Last Two Weeks",
        "bot.dashboard.hunter.time.ranges.lastMounth": "Last Month",
        "bot.dashboard.hunter.jobStatus": "Agent Working Status",
        "bot.dashboard.hunter.language": "Language",
        "bot.dashboard.hunter.turnType": "Agent Hand-off Type",
        "bot.dashboard.hunter.ok": "Search",

        "agent.hunter.time": "Time Range",
        "agent.hunter.ok": "Confirm",
}%
```

## See Also

`./gen_locale`无法执行，可能是改二进制文件未有执行权限
```
chmod 777 ./gen_locale  # 修改权限
```