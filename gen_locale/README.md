# gen_locale

> Generate locale `ts|json` files from Excel file

## TODO

- [x] Content newline issue, currently using string template
- [x] Generate locale files with different extension types, e.g., ts, json
- [x] Consider adding a parameter to specify the sheet name to be parsed in the Excel file; parameter -s --sheet is now added
- [x] Print error when sheet name not found in Excel file
- [x] Warnings for duplicate ids, errors, etc.

## How to use

```bash
$ cargo run -- -i file.xlsx
$ cargo run -- -i file.xlsx -d dist
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

`./gen_locale` is not executable, maybe the binary file does not have execution permission
```
chmod +x ./gen_locale  # make the binary executable
```