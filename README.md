# table

## Installation

```sh
$ brew tap naoty/misc
$ brew install table
```

## Input Format

### TSV

```sh
$ echo -e "2017-01-01\t10000\n2017-01-02\t8000" | table
+------------+-------+
| 2017-01-01 | 10000 |
| 2017-01-02 | 8000  |
+------------+-------+
```

`-H` or `--header` option adds headers to the table.

```sh
$ echo -e "day\tDAU\n2017-01-01\t10000\n2017-01-02\t8000" | table -H
+------------+-------+
| day        | DAU   |
+------------+-------+
| 2017-01-01 | 10000 |
| 2017-01-02 | 8000  |
+------------+-------+
```

### CSV

```sh
$ echo -e "2017-01-01,10000\n2017-01-02,8000" | table -f csv:ascii
+----+-----------+
| id | name      |
+----+-----------+
| 1  | bulbasaur |
| 2  | ivysaur   |
+----+-----------+
```

## Output Format

### ASCII

```sh
$ echo -e "day\tDAU\n2017-01-01\t10000\n2017-01-02\t8000" | table -H -f tsv:ascii
+------------+-------+
| day        | DAU   |
+------------+-------+
| 2017-01-01 | 10000 |
| 2017-01-02 | 8000  |
+------------+-------+
```

### Markdown

```sh
$ echo -e "day\tDAU\n2017-01-01\t10000\n2017-01-02\t8000" | table -H -f tsv:markdown
| day        | DAU   |
| ---------- | ----- |
| 2017-01-01 | 10000 |
| 2017-01-02 | 8000  |
```
