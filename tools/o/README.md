# o

Personal tool to keep track of things locations. The 'o' stands for "Organize".

## Planned initial set of features

- [x] Create CLI commands parser
- [x] Project initializer:
    - [x] Creates a directory `.o` with the following files: `.gitignore`, `o_data`, `o_config.toml`
- [x] Import and export from CSV format
    - [x] Populate imported data (ids, dates?)
- [x] CRUD: Create, Read, Update, Delete
    - [x] Populate missing data: ids, dates
- [ ] Stats (e.g. nums of items) display
    - [x] Count
    - [ ] Last creation
    - [ ] Last update
    - [ ] Items without container
- [ ] Search of entries via multiple approaches
- [ ] Data encryption / decryption supporting a configuration file
    - [ ] Put key (ignored by git) and configuration (not ignored by git) in different files
- [ ] Tree (hierarchy) display with several filters
- [ ] Possibility to UNDO latest N writes (edits, additions, deletions)

## Requirements

- It must be _really_ fast to use
- It must have a clear and customizable output
- Data must be easily ported and encrypted
- Ideally common output could be piped to tools like grep or fzf
