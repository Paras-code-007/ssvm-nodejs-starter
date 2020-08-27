# My Experience with the project

I am a sophomore doing my graduation in Computer Science and Engineering. I started web at my first year in the college and was always curious about the new and upcoming technologies in the feild. From the day i learnt that V8 was not the only the main thing in our browser and there was this web assembly thing which is very efficent and which can transpile any code written in any systems language into binary executables which can run on the browser and are very efficent and high performance, i saw this very appealing and the was confident to see the future of web development and where it will be going. Seeing the potential of web assembly through desktop applications like figma and autocad running fabulously and implemented over the browser i could understand the importance of such a technology and serverless computing.
I myself tried my hands once on web assembly using emscripten. But suddenly i read this article of being able to run the web assembly on the serverside instaed of the client side using second state virtual machine and was really cool.
I really wanna give heads off to the idea of virtualizing the web assembly environment from web browser and taking it to server side to get the advantages of web assembly and systems language on the server side as well to securely run a system app as a server.
I was fascinated about this but a hard time came when i tried to implement it.
Setting up the environment locally on my mac was difficult because of outaded system version of c++ on mac.
So i decided to learn docker and really understood its need and why it is important. 
And from that day docker became one of my favourite tools. We can do a lot of things with docker without really needing virtual machines and it also helps in deploying our app and moving the entire web app including everything data in databases easily from one cloud service to another
I can practice linux because who really needs a gui, and why to even create a virtual machine which can make my mac slow.
Since ssvm allows to use rust code on server side i had to learn rust which was already on my hit to go list because its way better and moder than cpp.
I already know cpp but i found rust more functional than cpp and with widespread approaches but i found it hard to get up with rust syntax and took a hard time figuring it out.
I was not able to get that well versed with rust because i need time to adjust with its syntax so i was only able tobuild really basic functions with it to be implement on ssvm.
Overall it was a great learning experience as it give me an approach how to grasp a language faster and debugging strategies to follow.


# Getting started

![Build and test](https://github.com/second-state/ssvm-nodejs-starter/workflows/Build%20and%20test/badge.svg)

[Fork this project](https://github.com/second-state/ssvm-nodejs-starter/fork) to create your own Rust functions in Node.js. [Learn more](https://www.secondstate.io/articles/getting-started-rust-nodejs-vscode/)

* The Rust functions are in the `src` directory. You can put high performance workload into Rust functions.
* The JavaScript functions are in the `node` directory and they can access the Rust functions.
* Use the `node node/app.js` command to run the application in Node.js.


## Use Docker to build and run

```
$ docker pull secondstate/ssvm-nodejs-starter:v1
$ docker run -p 3000:3000 --rm -it -v $(pwd):/app secondstate/ssvm-nodejs-starter:v1
(docker) # cd /app
(docker) # ssvmup build
(docker) # node node/app.js
```

From a second terminal window, you can test the local server.

```
$ curl -d '{"data":"helloworld"}' -H "Content-Type: application/json" -X POST http://localhost:3000/encode  
```


## Use VSCode Codespace

<p>
    <a href="https://online.visualstudio.com/environments/new?name=Rust%20and%20WebAssembly%20in%20Node.js&repo=second-state/ssvm-nodejs-starter">
        <img src="https://img.shields.io/endpoint?style=social&url=https%3A%2F%2Faka.ms%2Fvso-badge">
    </a>
</p>

![SSVM](https://github.com/second-state/blog/blob/master/static/images/SSVM-edited-without-music.gif?raw=true)

This project template works with the VS Codespaces online IDE! Code, build, and run directly from inside the browser. No software download or install needed! Check out the [high-res screencast](https://youtu.be/j85cbNsciOs).

> VS Codespaces runs entirely in your browser and costs around $1 per work day. It is cheaper than a cup of coffee in the office. Alternatively, use locally installed VSCode and Docker, and [launch the IDE with your remote git repository](https://code.visualstudio.com/remote-tutorials/containers/getting-started).

1 First, open the [VS Codespaces](https://online.visualstudio.com/) web site and login with your Azure account. You can get a [free Azure account](https://azure.microsoft.com/en-us/free/).

2 Next, create a new Codespace. Put your forked repository into the Git Repository field.

![Create a new Codespace](docs/img/vscode_create.png)

3 Then open the `src/lib.rs`, `node/app.js` and `Cargo.toml` files and see how the Node.js express app calls the Rust function to say hello.

![Code in Codespace](docs/img/vscode_code.png)

4 Click on the Run button on the left panel, and then the Launch Program at the top to build and run the application.

![Build and run](docs/img/vscode_run.png)

The Terminal window at the bottom shows the build progress. It builds the Rust program, and then launches the Node.js app.

![Build](docs/img/vscode_build.png)

The Debug window shows the Node.js server running and waiting for web requests.

![Debug](docs/img/vscode_debug.png)

5 Now, you have two choices. You could use the proxy link for `127.0.0.1:3000` to access the running server in a browser.

![Browser link](docs/img/vscode_port.png)

Or, you could open another terminal window in the IDE via the `Terminal -> New Terminal` menu.

![Open Terminal](docs/img/vscode_terminal.png)

From the terminal window, you can test the local server.

```
$ curl -d '{"data":"helloworld"}' -H "Content-Type: application/json" -X POST http://localhost:3000/encode
```

## Read more:

* [The Case for WebAssembly on the Server-side](https://www.secondstate.io/articles/why-webassembly-server/)
* [Guide on how to Rust and WebAssembly for server-side apps](https://www.secondstate.io/articles/getting-started-with-rust-function/)

## Resources

* [The Second State VM (SSVM)](https://github.com/second-state/ssvm) is a high performance [WebAssembly virtual machine](https://www.secondstate.io/ssvm/) designed for server-side applications.
* [The SSVM NPM addon](https://github.com/second-state/ssvm-napi) provides access to the SSVM, and programs in it, through a Node.js host application.
* [The SSVM ready tool, ssvmup](https://github.com/second-state/ssvmup) is a [toolchain](https://www.secondstate.io/articles/ssvmup/) for compiling Rust programs into WebAssembly, and then make them accessible from JavaScripts via the SSVM.

Brought to you by the Open source dev team at [Second State](https://www.secondstate.io/). Follow us on [Twitter](https://twitter.com/secondstateinc), [Facebook](https://www.facebook.com/SecondState.io/), [LinkedIn](https://www.linkedin.com/company/second-state/), [YouTube](https://www.youtube.com/channel/UCePMT5duHcIbJlwJRSOPDMQ), or [Medium](https://medium.com/wasm)

