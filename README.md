# table

## Installation

### stable
Visit [Latest release page](https://github.com/naoty/table/releases/latest) and download a binary.

### master

```sh
$ go get github.com/naoty/table
```

## Usage

```sh
$ echo "2017-01-01\t10000\n2017-01-02\t8000" | table
+------------+-------+
| 2017-01-01 | 10000 |
| 2017-01-02 | 8000  |
+------------+-------+
```

```sh
$ echo "day\tDAU\n2017-01-01\t10000\n2017-01-02\t8000" | table -H
+------------+-------+
| day        | DAU   |
+------------+-------+
| 2017-01-01 | 10000 |
| 2017-01-02 | 8000  |
+------------+-------+
```
