# Git TODO
Manage TODO lists for different branches

## Usage
```
Usage: git-todo [OPTIONS] <COMMAND>

Commands:
  add       Add a new TODO item
  check     Checks if there are still TODO items undone
  clear     Clears TODO items
  complete  Complete TODO items
  delete    Delete TODO items
  list      List TODO items
  help      Print this message or the help of the given subcommand(s)

Options:
  -b, --branch <BRANCH>  The branch to operate on
  -r, --repo <REPO>      Git repository path [default: .]
  -h, --help             Print help
```

## Internals
Internally the TODO items will be stored under `.git/todo.json`.

## Examples
### Add New TODO Item
```bash
git todo add "Implement secret feature"
```

### List TODO Items
This will show the TODO along with completion and internal ID.
```bash
git todo list
```

### Delete TODO Items
Pass one or more IDs to delete.
```bash
git todo delete 0
```

### Clear TODO Items
```bash
git todo clear
```

### Clear Completed TODO Items
```bash
git todo clear --done
```

### Complete TODO Items
Pass one or more IDs to complete.
```bash
git todo complete 0
```

### Check TODO Status
Returns 0/1 depending on if all items are completed, useful for scripting and hooks.
```bash
git todo check --quiet
```
