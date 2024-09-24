# User Stories

1. When a User runs resume generator with no
   arguments, a hello message is printed to
   stdout sharing available commands &
   options.

   ```
   $ rgen
   Resume builder & output generator.

   Usage: rgen [OPTIONS] [COMMAND]

   Options:
   -f, --save-file    path to sqlite db file to use
   -?, --help         disply this message

   Commands:
   init <file path>   create new save file
   add <subcommand>   add data, see `rgen add --help` for more
   show <subcommand>  show data, see `rgen show --help` for more
   edit <subcommand>  edit data, see `rgen edit --help` for more
   del <subcommand>   delete data, see `rgen del --help` for more
   create <file path> generate new resume of given type at given path, see `rgen create --help` for more
   ...
   ```

<!-- FIXME -->

2. When a User runs resume generator with
   the ...?

<!-- TODO -->

- [ ] something to view existing data
  ```
  
  ```
- [ ] something to create new data
  ```
  ```
- [ ] something to edit existing data
  ```
  ```
- [ ] a flow to generate a resume by
  1. asking for jd info, such as key
     words, title, etc.
  2. prompting to select items for each
     section from existing data, filtered
     or sorted by relevance to info given
     in (1)
  3. rendering data to output per
     templates
  ```
  ```
