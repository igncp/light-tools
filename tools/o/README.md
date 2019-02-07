# o

Personal tool to keep track of things locations. The 'o' stands for "Organize".

## Planned features

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
- [ ] Tree (hierarchy) display with several filters
    - [x] Tree display
    - [ ] Choose starting node
- [ ] List entries
    - [x] List entries by order of creation (default)
    - [ ] List entries by order of update
    - [ ] List entries by hierarchy order
- [ ] Search of entries via multiple approaches
    - [x] Search by string
    - [x] Search by id
    - [ ] Search by updated in range
    - [ ] Search by created in range
- [ ] Possibility to UNDO latest N writes (edits, additions, deletions)
    - [x] Create a `.o/backups` directory
    - [x] Create N (configurable) files of backup that are updated when data changes
    - [x] Support using a backup file by using the `rev` subcommand
- [ ] Data encryption / decryption supporting a configuration file
    - [ ] Put key (ignored by git) and configuration (not ignored by git) in different files
    - [ ] Encrypt data file when writing and decrypt when reading

## Requirements

- It must be _really_ fast to use
- It must have a clear and customizable output
- Data must be easily ported and encrypted
- Ideally common output could be piped to tools like grep or fzf
