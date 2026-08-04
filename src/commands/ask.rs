use crate::{Context, Error, i18n};
use poise::CommandParameterChoice;
use crate::utils::ContextExt;

#[derive(Debug)]
#[derive(PartialEq)]
pub enum Question {
    AreYouFemboy,
    LinuxTheBest
}

#[poise::command(slash_command)]
pub async fn ask(ctx: Context<'_>, question: Question) -> Result<(), Error> {
    match question {
        Question::AreYouFemboy => ctx.reply_locale("ask.yes").await?,
        Question::LinuxTheBest => ctx.reply_locale("ask.gnu_linux").await?
    };

    Ok(())
}

impl poise::ChoiceParameter for Question {
    fn list() -> Vec<CommandParameterChoice> {
        vec![
            CommandParameterChoice {
                name: i18n::get("en-US", "ask.are_you_femboy").to_string(),
                localizations: i18n::get_all("are_you_femboy"),
                __non_exhaustive: (),
            },
            CommandParameterChoice {
                name: i18n::get("en-US", "ask.linux_the_best").to_string(),
                localizations: i18n::get_all("ask.linux_the_best"),
                __non_exhaustive: (),
            },
        ]
    }

    fn from_index(index: usize) -> Option<Self> {
        match index {
            0 => Some(Self::AreYouFemboy),
            1 => Some(Self::LinuxTheBest),
            _ => None,
        }
    }

    fn from_name(_name: &str) -> Option<Self> {
        None
    }

    fn name(&self) -> &'static str {
        ""
    }

    fn localized_name(&self, locale: &str) -> Option<&'static str> {
        match self {
            Self::AreYouFemboy => i18n::get_option(locale, "ask.are_you_femboy"),
            Self::LinuxTheBest => i18n::get_option(locale, "ask.linux_the_best")
        }
    }
}
