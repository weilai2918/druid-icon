# druid-icon
在Windows上，如果可执行文件包含一个id为1的图标资源，这个图标将被用在Druid窗口的任务栏和标题栏中。如果没有这样的图标资源，则将使用默认的应用程序图标。
添加自定义图标：一种方法是使用winres crate并遵循其自述文件中的指南;winres::WindowsResource::set_icon() 添加ID为1的图标。
创建druid项目设置窗体图标
```bash
cargo new druid-icon
```
添加druid包
```toml
[dependencies]
druid = "0.8.2"
```
添加设置图标使用的库
```toml
[package]
build = "build.rs"

[build-dependencies]
winres = "0.1.12"
```
添加build.rs启动文件 (项目根目录下)
```rust
extern crate winres;

fn main() {
    if cfg!(target_os = "windows") {
        let mut res = winres::WindowsResource::new();
        //设置图标，图标和build.rs文件都放在项目根目录下
        res.set_icon("logo.ico");
        res.compile().unwrap();
    }
}
```
编写main.rs主方法
```rust
use druid::widget::{Flex, Label};
use druid::{AppLauncher, PlatformError, Widget, WindowDesc, Data, Lens};

#[derive(Data,Lens,Clone)]
struct AppData{

}

fn main() -> Result<(), PlatformError> {
    //创建窗体，并且设置为300x250
    let main_window = WindowDesc::new(ui_builder()).window_size((300.,250.));
    //启动
    AppLauncher::with_window(main_window)
        //打印日志log
        .log_to_console()
        .launch(AppData{})
}

fn ui_builder() -> impl Widget<AppData> {
    
    //创建显示的内容
    Flex::<AppData>::column()
        .with_child(Label::new("设置图标"))
}

```
运行效果
![image.png](https://cdn.nlark.com/yuque/0/2023/png/22289421/1677056924209-5a457a06-2169-4e55-be02-548e6da27d46.png#averageHue=%23c3b9ab&clientId=u617e27e6-4a82-4&from=paste&height=310&id=u6b1a8a3d&name=image.png&originHeight=310&originWidth=316&originalType=binary&ratio=1&rotation=0&showTitle=false&size=35920&status=done&style=none&taskId=ue4aedb56-4b54-442c-810a-bd4ed620fe2&title=&width=316)
项目目录
![image.png](https://cdn.nlark.com/yuque/0/2023/png/22289421/1677056966608-37e6bcc8-ff90-4cd5-a209-00249181d95a.png#averageHue=%2327282a&clientId=u617e27e6-4a82-4&from=paste&height=245&id=u517f6684&name=image.png&originHeight=245&originWidth=304&originalType=binary&ratio=1&rotation=0&showTitle=false&size=12432&status=done&style=none&taskId=u68ed411a-ae4c-46f0-8686-26c4333b252&title=&width=304)
github地址:
