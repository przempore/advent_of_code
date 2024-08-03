#include <iostream>
#include <ftxui/component/component.hpp>
#include <ftxui/component/screen_interactive.hpp>

int main() {
    auto screen = ftxui::ScreenInteractive::FitComponent();
    auto slider = ftxui::Slider(ftxui::ConstStringRef("Slider"), ftxui::Ref<int>(50), ftxui::ConstRef<int>(0), ftxui::ConstRef<int>(100), ftxui::ConstRef<int>(25));

    auto component = ftxui::Renderer(slider, [&] {
        return ftxui::vbox({
            ftxui::text("Hello"),
            ftxui::hbox({
                ftxui::text("World"),
                slider->Render(),
            }),
        });
    });

    screen.Loop(component);
}
