**Roadmap:**

usage:
```bash
# to read json in rusty way
jsoneer example.json 
# to replace every value with "placeholder"
jsoneer -u placeholder example.json 
# to replace with null,false,true,[],{} respectfully 
jsoneer -[n|f|t|a|o] placeholder example.json 
```


- [x] Replace all values in the json by 'null','false','true', '{}', '[]' and user-specified
- [x] Support filenames as input
- [x] Move to "clap" for parsing
- [ ] Raplce values one-by-one, replace with 'shifting', or by index **IN PROGRESS**
- [ ] Generate payloads based on the ruleset
- [ ] Host web version
- [ ] Create simple GUI/TUI
- [ ] Implemet itegration with some QA tooling
