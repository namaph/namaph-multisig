use crate::error::NamaphError;
use crate::model::text::TextTopic;
use crate::util::fit_string;
use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(proposer: Pubkey, title: String, body:String)]
pub struct UpdateTextTopic<'info> {
    #[account(
        mut,
        seeds = [b"text", text_topic.multisig.as_ref(), fit_string(&text_topic.title)],
        bump=text_topic.bump,
        constraint=text_topic.proposer == proposer
              )]
    text_topic: Account<'info, TextTopic>,
    #[account(mut)]
    authority: Signer<'info>,
}

pub fn handle(ctx: Context<UpdateTextTopic>, _proposer: Pubkey, _title: String, body: String) -> Result<()> {
    require!(
        body.len() < TextTopic::MAX_BODY_BYTES,
        NamaphError::StringTooLong
    );

    let text_topic = &mut ctx.accounts.text_topic;
    text_topic.body = body;
    Ok(())
}
