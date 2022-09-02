# pmtiles_tool

A simple tool to work with mbtiles and pmtiles archives.

This tool was created because the Python tool uses a lot of memory and takes a long time
to process large tilesets. This tool is optimized for memory and uses multiple threads
to decompress tiles.

Currently implemented subcommands:

- [x] convert (convert mbtiles to pmtiles)
- [x] info (show statistics of a pmtiles archive)
- [x] serve (serve tiles from a pmtiles archive)

Run `pmtiles_tool help` for more information:

```
$ pmtiles_tool help
pmtiles_tool
A tool for working with pmtiles archives

USAGE:
    pmtiles_tool <SUBCOMMAND>

OPTIONS:
    -h, --help    Print help information

SUBCOMMANDS:
    convert    Convert a mbtiles archive to a pmtiles archive
    help       Print this message or the help of the given subcommand(s)
    info       Get information about a pmtiles archive
    serve      Serve XYZ tiles from a pmtiles archive
```

Each subcommand has its own help page:

```
$ pmtiles_tool help convert
pmtiles_tool-convert
Convert a mbtiles archive to a pmtiles archive

USAGE:
    pmtiles_tool convert <INPUT> <OUTPUT>

ARGS:
    <INPUT>     Input
    <OUTPUT>    Output

OPTIONS:
    -h, --help    Print help information
```

## Install
リリースページからダウンロード

```
$ mv pmtiles_tool ~/bin/pmtiles_tool
$ chmod 755

// 確認
$ pmtiles_tool help
```


## Preview PMTiles Content

```
$ pmtiles_tool serve sample.pmtiles
Starting server on port 8888
```

QGIS で確認
![screenshot_from_2022-09-02_10-06-07](https://user-images.githubusercontent.com/8760841/188110209-03b13b5e-eb68-4187-b07e-6b283c46e9cc.png)

