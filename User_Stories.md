# User Stories

1. When a User runs resume generator with no arguments, a hello message is printed to stdout sharing available commands & options.

   ```
   $ res-gen
   Resume builder & output generator.

   Usage: res-gen [OPTIONS] [COMMAND]
          res-gen init <PATH>

   Options:
   -f, --save-file    path to sqlite db file to use
   -?, --help         disply this message

   Commands:
   init <PATH>        create new save file
   add [SUBCOMMAND]   add data, see `res-gen add --help` for more
   edit [SUBCOMMAND]  edit data, see `res-gen edit --help` for more
   export [OPTIONS]   export a resume as one of the supported file types
                      see `res-gen export --help` for more
   show <DATA-TYPE>   quickly print requested data to stdout, including all
                      saved jobs, resumes, projects, etc.
                      see `res-gen show --help` for more
   ```

2. When a User runs init...

   1. they must give a path to save the datafile at.
   2. they are then asked

      3. if they want to start adding data via an interactive prompt
      4. or if they want to exit

3. When a User runs add...

   1. they can give a subcommand to skip to the desired menu item
   2. if they provide no subcommands, they are shown a menu for interactively adding a...

      1. resume
      1. job
      1. skill
      1. project
      1. or education item to the system.

   3. When a User selects a menu item or provides the associated subcommand, they are taken to the associated editing story starting with a blank template.
   4. When a User completes adding the requested item via the associated editor, they are taken back to the menu if they started at it, else the program exits.

   ```
   $ res-gen add
   Resume builder & output generator.

   Usage: res-gen add [OPTIONS] [SUBCOMMAND]

   Options:
   -?, --help         disply this message

   Commands:
   resume [NAME]      add a resume, prompting for a resume name if not
                      given
   job [NAME]         add a job, prompting for a job name if not given
   skill [NAME]       add a skill, prompting for a skill name if not given
   project [NAME]     add a project, prompting for a project name if not
                      given
   education [NAME]   add an education item, prompting for an education name
                      if not given
   ```

4. When a user runs edit with the

   1. They

   ```
   $ res-gen edit
   Resume builder & output generator.

   Usage: res-gen edit [OPTIONS] [SUBCOMMAND]

   Options:
   -?, --help         disply this message

   Commands:
   resume [id]        edit a resume, prompting for a resume name/id if not
                      given or if matching name/id is not found
   job [id]           edit a job, prompting for a job name/id if not given or if
                      matching name/id is not found
   skill [id]         edit a skill, prompting for a skill name/id if not given
                      or if matching name/id is not found
   project [id]       edit a project, prompting for a project name/id if not
                      given or if matching name/id is not found
   education [id]     edit an education item, prompting for an education name/id
                      if not given or if matching name/id is not found
   ```
