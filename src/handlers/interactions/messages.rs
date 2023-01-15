/*
 * Copyright (c) 2022 tooboredtocode
 * All Rights Reserved
 */

use crate::util::discord_locales::DiscordLocale;
use twilight_model::user::User;

#[inline]
pub const fn missing_permissions(locale: DiscordLocale, pinned: bool) -> &'static str {
    match (locale, pinned) {
        (DiscordLocale::GERMAN, true) => {
            "Ich konnte die nachricht nicht anpinnen, da ich keine Berechtigung habe, \
            Nachrichten zu bearbeiten."
        }
        (DiscordLocale::GERMAN, false) => {
            "Ich konnte die nachricht nicht entpinnen, da ich keine Berechtigung habe, \
            Nachrichten zu bearbeiten."
        }
        (_, true) => {
            "I couldn't pin the message, because I don't have the permission to edit messages."
        }
        (_, false) => {
            "I couldn't unpin the message, because I don't have the permission to edit messages."
        }
    }
}

#[inline]
pub const fn error(locale: DiscordLocale) -> &'static str {
    match locale {
        DiscordLocale::GERMAN => {
            "Ein unerwarteter Fehler ist passiert, die Developer wurden benachrichtigt"
        }
        _ => "An unexpected error has occurred, the dev team has been alerted",
    }
}

#[inline]
pub fn audit_log_reason(locale: DiscordLocale, user: Option<&User>) -> String {
    match (locale, user) {
        (DiscordLocale::GERMAN, Some(user)) => {
            format!("Pin Befehl durch Benutzer {} <@{}>", user.name, user.id)
        }
        (DiscordLocale::GERMAN, None) => "Pin Befehl durch unbekannten Benutzer".to_string(),
        (_, Some(user)) => {
            format!("Pin Command execution by user {} <@{}>", user.name, user.id)
        }
        (_, None) => "Pin Command execution by an unknown user".to_string(),
    }
}
