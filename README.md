# Description
Utility to generate Master Duel banlist for EDOPro simulator

### CLI Options
```
Usage: ygo-utils.exe <--show <SHOW>|--generate <GENERATE>>

Options:
  -s, --show <SHOW>          [possible values: all, current, next]
  -g, --generate <GENERATE>  
  -h, --help                 Print help
  -V, --version              Print version
```

`show` will show all available Master Duel banlists.
They are named by their effective date in YYYY-MM-DD format

`generate` will generate the conf file for the specified banlist. Banlists are specified by the effective date in YYYY-MM-DD format