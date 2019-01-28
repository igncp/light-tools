# o

Personal tool to keep track of things locations. The 'o' stands for "Organize".

## Planned initial set of features

- [x] Create CLI commands parser
- [x] Project initializer:
    - Creates a directory `.o` with the following files: `.gitignore`, `o_data`, `o_config.toml`
- [ ] Import and export from CSV format
    - Populate imported data (ids, dates?)
- [ ] CRUD: Create, Read, Update, Delete
    - Populate missing data: ids, dates
- [ ] Search of entries via multiple approaches
- [ ] Stats (e.g. nums of items) display
- [ ] Data encryption / decryption supporting a configuration file
- [ ] Tree (hierarchy) display with several filters

## Requirements

- It must be _really_ fast to use
- It must have a clear and customizable output
- Data must be easily ported and encrypted
- Ideally common output could be piped to tools like grep
