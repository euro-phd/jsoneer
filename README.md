![example workflow](https://github.com/euro-phd/jsoneer/actions/workflows/rust.yml/badge.svg)
# Usage:
```bash
# to read json in rusty way
jsoneer show example.json 
# to replace every value with "placeholder"
jsoneer set example.json -m u placeholder 
# to replace with null,false,true,[],{} respectfully 
jsoneer set example.json -m [n|f|t|a|o] 
```

# Roadmap:
- [x] Replace all values in the json by 'null','false','true', '{}', '[]' and user-specified
- [x] Support filenames as input
- [x] Move to "clap" for parsing
- [ ] Raplce values one-by-one, replace with 'shifting', or by index **IN PROGRESS**
- [ ] Generate payloads based on the ruleset
- [ ] Host web version
- [ ] Create simple GUI/TUI
- [ ] Implemet itegration with some QA tooling
