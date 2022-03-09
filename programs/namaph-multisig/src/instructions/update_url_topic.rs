use crate::error::NamaphError;
use crate::model::url::UrlTopic;
use crate::util::fit_string;
use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(proposer:Pubkey, title: String, url:String)]
pub struct UpdateUrlTopic<'info> {
    #[account(
        mut,
        seeds = [b"url", url_topic.multisig.as_ref(), fit_string(&url_topic.title)],
        bump=url_topic.bump,
        constraint = url_topic.proposer == proposer
              )]
    url_topic: Account<'info, UrlTopic>,
    #[account(mut)]
    authority: Signer<'info>,
}

pub fn handle(ctx: Context<UpdateUrlTopic>, _proposer:Pubkey, _title: String, url: String) -> Result<()> {
    require!(
        url.len() < UrlTopic::MAX_URL_BYTES,
        NamaphError::StringTooLong
    );

    let url_topic = &mut ctx.accounts.url_topic;
    url_topic.url = url;
    Ok(())
}
