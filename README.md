# regui
 A React inspired gui library

:warning: this is a work in progress (very early stages), and is not yet ready for use.

## Quickstart

At the moment, the only the iui and nwg kits are somewhat usable.  
Try one of the examples:
```sh
cargo run -p regui-iui --example async
cargo run -p regui-iui --example basic
cargo run -p regui-iui --example controls
```

## Provided kits

In order of priority:

| Implementation | Status | Notes |
| --- | --- | --- |
| [`regui-iui`](./kits/iui/) | :construction: | based on [`iui`](https://github.com/rust-native-ui)
| [`regui-nwg`](./kits/nwg/) | :construction: | based on [`native-windows-gui`](https://github.com/gabdube/native-windows-gui)
| [`regui-web`](./kits/web/) | :x: :star: | based on [`web-sys`](https://rustwasm.github.io/wasm-bindgen/web-sys/index.html)
| [`regui-repaint`](./kits/repaint/) | :x: | custom based on [repaint](https://github.com/re-gui/repaint)
> :warning: TODO other kits. Possibly some wrappers for [other gui libraries](https://www.areweguiyet.com/)

Legend:
- :star: : (will be) the reference implementation
- :x: : not started
- :construction: : in progress
- :white_check_mark: : done

<!--
TODO:
 - [ ] web, yew like
 - [ ] custom based on repaint
 - [ ] druid
 - [ ] gtk
 - [ ] Qt
 - [ ] wxWidgets
 - [ ] modern windows api
 - [ ] android
 - [ ] ios
 - [ ] macos
 - [ ] linux


## Comunity provided kits

_Empty_
-->