use regex::Regex;

use super::*;

/// Control the blocklist
#[command]
#[sub_commands(blocklist_add, blocklist_remove, blocklist_get)]
pub async fn blocklist(_ctx: &client::Context, _msg: &Message) -> CommandResult {
    abort_with!(UserErr::invalid_usage(&BLOCKLIST_COMMAND_OPTIONS));
}

#[command("add")]
#[usage("blocklist add `regex`")]
pub async fn blocklist_add(ctx: &client::Context, msg: &Message, args: Args) -> CommandResult {
    let data = ctx.data.read().await;
    let db = data.get::<Db>().unwrap().clone();

    let pattern = args
        .remains()
        .filter(|x| x.starts_with('`') && x.ends_with('`'))
        .invalid_usage(&BLOCKLIST_COMMAND_OPTIONS)?;

    // verified previously
    let pattern = pattern
        .strip_prefix('`')
        .and_then(|x| x.strip_suffix('`'))
        .unwrap();

    let _ = Regex::new(&pattern).user_error("Illegal regex pattern")?;

    db.add_blocklist_entry(msg.author.id, &pattern).await?;

    msg.reply_success(&ctx, format!("Added `{}` to the blocklist", pattern))
        .await?;

    Ok(())
}

#[command("remove")]
#[usage("blocklist remove `regex`")]
pub async fn blocklist_remove(ctx: &client::Context, msg: &Message, args: Args) -> CommandResult {
    let data = ctx.data.read().await;
    let db = data.get::<Db>().unwrap().clone();

    let pattern = args
        .remains()
        .filter(|x| x.starts_with('`') && x.ends_with('`'))
        .invalid_usage(&BLOCKLIST_COMMAND_OPTIONS)?;

    // verified previously
    let pattern = pattern
        .strip_prefix('`')
        .and_then(|x| x.strip_suffix('`'))
        .unwrap();

    db.remove_blocklist_entry(pattern).await?;
    msg.reply_success(&ctx, format!("Removed `{}` from the blocklist", pattern))
        .await?;

    Ok(())
}

#[command("get")]
#[usage("blocklist get")]
pub async fn blocklist_get(ctx: &client::Context, msg: &Message) -> CommandResult {
    let data = ctx.data.read().await;
    let db = data.get::<Db>().unwrap().clone();

    let entries = db.get_blocklist().await?;

    msg.reply_embed(&ctx, |e| {
        e.description(entries.iter().map(|x| format!("`{}`", x)).join("\n"));
    })
    .await?;
    Ok(())
}
