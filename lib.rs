use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_instruction;
// This is your program's public key and it will update
// automatically when you build the project.
declare_id!("8F7mNbgH5LZC5hhSiyKch8c6vKVAaTZGCNqWsGRTKHtL");

#[program]
pub mod solar_tips {
    use super::*;

    pub fn enviar_propina(ctx: Context<EnviarPropina>, monto: u64) -> Result<()> {
        let instruccion = system_instruction::transfer(
            &ctx.accounts.donador.key(),
            &ctx.accounts.receptor.key(),
            monto,
        );

        anchor_lang::solana_program::program::invoke(
            &instruccion,
            &[
                ctx.accounts.donador.to_account_info(),
                ctx.accounts.receptor.to_account_info(),
            ],
        )?;

        // Comentamos esto para que no busque la cuenta que no está inicializada
        // let registro = &mut ctx.accounts.registro_propina;
        // registro.ultimo_donador = *ctx.accounts.donador.key;
        // registro.monto_total += monto;

        msg!("Propina enviada con exito. ¡Gracias por su apoyo!");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct EnviarPropina<'info> {
    // We must specify the space in order to initialize an account.
    // First 8 bytes are default account discriminator,
    // next 8 bytes come from NewAccount.data being type u64.
    // (u64 = 64 bits unsigned integer = 8 bytes)
    pub registro_propina: UncheckedAccount<'info>, 
    #[account(mut)]
    pub donador: Signer<'info>,
    #[account(mut)]
    /// CHECK: Cuenta receptora
    pub receptor: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Registro {
    pub ultimo_donador: Pubkey,
    pub monto_total: u64,
}
