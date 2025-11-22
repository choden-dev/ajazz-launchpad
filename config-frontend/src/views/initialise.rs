use crate::messages::Messages;

pub struct Initialise;
impl Initialise {
    pub fn view(&'_ self) -> iced::Element<'_, Messages> {
        iced::widget::column![iced::widget::text("Loading...")]
            .padding(10)
            .spacing(10)
            .into()
    }
}
