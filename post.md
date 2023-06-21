Just started working on a new GUI library

I recently started working on a new GUI library, [`regui`](https://github.com/re-gui/regui). The project is still in its very early stages and (currently) only runs on windows (sorry :disappointed:), but I'm already looking for feedback!

The main goal of this project is to provide a simple, easy to use, and easy to understand GUI library inspired by the React architecture. I'm trying to make it as simple as possible, while still being powerful enough to build complex applications. You can see an example of what it looks like [here](https://github.com/re-gui/regui/blob/main/kits/nwg/examples/basic_functional.rs).

The internal implementation is a real mess, but I'm working on it and I'm confident I can make it much better, if there is enough interest in the project.

I'm also working on [`repaint`](https://github.com/re-gui/repaint) that I plan to use to provide a simple way to draw on canvas or to create custom widgets.

I'm looking for feedback on the project, especially on the API and the architecture. I'm also looking for help with the implementation, so if you are interested in the project, feel free to open an issue/discussion.

---

Some notes:
- It currently runs on windows only because I'm using [`native-windows-gui`](https://github.com/gabdube/native-windows-gui) in the only kit I'm currently implementing. This because I work on windows and I thought this was the easiest way to get started. I'm planning to implement other kits: I'm especially interested in creating a web kit and a Qt wrapper kit in the near future.

Example:
 
![example](https://i.imgur.com/220bTRc.png)