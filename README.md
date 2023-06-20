# regui
 A React inspired gui library

:warning: this is a work in progress (very early stages), and is not yet ready for use.

## Quickstart

At the moment, the only decent exmple is the [basic_functional](./kits/nwg/examples/basic_functional.rs) example.
```sh
cargo run -p regui-nwg --example basic_functional
```

## Provided kits

In order of priority:

| Implementation | Status | Notes |
| --- | --- | --- |
| [`regui-nwg`](./kits/nwg/) | :construction: | based on [`native-windows-gui`](https://github.com/gabdube/native-windows-gui)
| [`regui-web`](./kits/web/) | :x: :star: | based on [`web-sys`](https://rustwasm.github.io/wasm-bindgen/web-sys/index.html)
| ??? | :x: | custom based on [repaint](https://github.com/re-gui/repaint)
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