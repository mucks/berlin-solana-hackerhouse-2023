use anchor_lang::prelude::*;

#[event]
pub struct LessonEnded {
    teacher: u32,
    lesson: u32,
    student: u32,
}

#[derive(Accounts)]
#[instruction(
teacher: u32,
lesson: u32,
student: u32,
)]
pub struct LessonEnd<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    pub rent: Sysvar<'info, Rent>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<LessonEnd>, teacher: u32, lesson: u32, student: u32) -> Result<()> {
    emit!(LessonEnded {
        teacher,
        lesson,
        student
    });

    Ok(())
}
