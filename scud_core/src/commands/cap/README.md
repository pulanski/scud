# Cap

Cap - *Commit all and push*

Condenses the processes of staging, committing, and pushing into a single command.

Running `scud cap` is analogous to sequentially running either
* `scud stage`, `scud commit`, and `scud push`

or

* `scud commit-all` and `scud push`.

NOTE:

TODO: add potential notes here if needed

## Usage

### Default Usage

```
scud cap
```

### Usage with alias

No alias is provided for this command.

## Example

```
scud cap
```

<!-- Provide a usage video example of the command -->

## Under the hood

<!-- TODO under the hood command can be accessed via scud cap -i -->

<!-- scud cap -->
<!-- Under the hood: -->
<!--     scud commit-all -->
<!--     scud push -->

As described above, `scud cap` is a shortcut for running `scud commit-all` and `scud push`.