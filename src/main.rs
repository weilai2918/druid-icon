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
