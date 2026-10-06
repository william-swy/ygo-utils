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

`generate` will generate the conf file for the specified banlist. Banlists are specified by the effective date in YYYY-MM-DD format. This will generate a file
with the name `<YYYY-MM-DD>_master_duel.lflist.conf`. Place this file under `<EDOPro install location>/repositories/lflists`.

### Output Snippet
```
#[2026-09-03 Master Duel]
!2026-09-03 Master Duel
#Forbidden
35059553 0 --Kaiser Colosseum
78706415 0 --Fiber Jar
88581108 0 --True King of All Calamities
8903700 0 --Djinn Releaser of Rituals
59750328 0 --Card of Demise
88071625 0 --The Tyrant Neptune
76815942 0 --Lyrilusc - Independent Nightingale
58921041 0 --Anti-Spell Fragrance
28566710 0 --Last Turn
43262273 0 --Appointer of the Red Lotus
87639778 0 --Harpie's Feather Storm
```