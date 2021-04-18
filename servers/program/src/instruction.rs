// //! Instruction types

use borsh::{BorshDeserialize, BorshSchema, BorshSerialize};
use num_derive::ToPrimitive;
use solana_program::{
    instruction::AccountMeta, program_error::ProgramError, pubkey::Pubkey, system_program, sysvar,
};

/// Instructions
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone, BorshSchema, ToPrimitive)]
pub enum Instruction {
    /// Create derived account
    ///
    /// Input: [AddressTypeInput]
    CreateDerivedAccount,

    /// [initialize_dweller]
    /// accounts
    /// - signer, write     dweller
    ///
    /// Input:
    ///  [InitializeDwellerInput]
    InitializeDweller,

    /// Initializes server and joins dweller_owner
    /// accounts
    /// - signer,  write          dweller_owner
    /// - signer,  write          server
    /// - derived, write          dweller_server
    /// - derived, write          server_member
    /// Input: [InitializeServerInput]
    InitializeServer,

    /// Change dweller's display name
    ///
    /// Accounts:
    /// - write, signer     dweller
    /// Input: [SetNameInput]
    SetDwellerName,

    /// Change dweller's display photo. Consider using PNG or JPEG photos for usability.
    ///
    /// Accounts:
    /// - signer, write   dweller
    ///
    /// Input: [SetHashInput]
    SetDwellerPhoto,

    /// Update the users status
    ///
    /// Accounts:
    /// - signer, write   dweller owner
    ///
    /// Input: [SetDwellerStatusInput]
    SetDwellerStatus,

    /// Initialize channel and add it to server.
    ///
    /// Accounts:
    /// - signer             dweller
    /// - read, derived      server_administrator with dweller and server seeds
    /// - write              server
    /// - write, derived     server_channel
    ///
    /// Input:
    /// [AddChannelInput]
    AddChannel,

    /// Accounts:
    /// - signer                 dweller
    /// - read, derived          server_administrator
    /// - write                  server
    /// - write, derived         server_channel
    /// - write, derived         server_channel_last
    DeleteChannel,

    /// Initialize group and add to server.
    ///
    /// Accounts:
    /// - signer            dweller
    /// - read, derived     server_administrator with dweller
    /// - write             server
    /// - write, derived    server_group
    ///
    /// Input:
    /// - [CreateGroupInput]
    CreateGroup,

    /// Accounts:
    /// - signer    dweller    
    /// - read      server_administrator
    /// - write     server
    /// - write     server_group
    /// - write     server_group_last
    /// - write     [group_channel] all channels in group
    DeleteGroup,

    /// Accounts:
    /// - write     server
    /// - signer    dweller
    /// - read      server_administrator
    /// - write     group_channel
    /// - read      channel
    AddChannelToGroup,

    /// Accounts:
    /// - write     server
    /// - signer    dweller
    /// - read      server_administrator
    /// - read      channel
    /// - write     group_channel
    /// - write     group_channel_last
    RemoveChannelFromGroup,

    /// Accounts:
    ///
    /// - signer    owner of server
    /// - read      dweller to become admin
    /// - write     server
    /// - write     server_administrator
    AddAdmin,

    /// Accounts:
    /// - signer    owner
    /// - write     server
    /// - write     admin
    /// - write     admin_last
    RemoveAdmin,

    /// Accounts:
    ///   - writeable         server     
    ///   - read signer       dweller
    ///   - writeable         dweller_server
    ///   - writeable         server_member
    ///   - read              server_member_status
    JoinServer,

    /// Accounts:
    ///
    /// - write, signer     dweller
    /// - write             server
    /// - write             server_member
    /// - write             server_member_last
    /// - write             dweller_server
    /// - write             dweller_server_last
    LeaveServer,

    /// Accounts:
    /// - write             server
    /// - signer            dweller_administrator
    /// - read, derived     server_administrator
    /// - read              dweller
    /// - write             member_status
    InviteToServer,

    /// Accounts:
    /// - read, signer       admin
    /// - write              server
    /// - write, derived     member_status
    /// - write, derived     member_status_last
    RevokeInviteServer,

    /// Accounts:
    /// - signer   admin
    /// - write    server
    ///
    /// Input: [SetNameInput]
    SetServerName,

    /// Accounts:
    /// - signer    admin
    /// - write     server
    ///
    /// Input: [SetHashInput]        
    SetServerDb,
}

/// Address type
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone, BorshSchema)]
pub enum AddressTypeInput {
    DwellerServer(u64),
    ServerMemberStatus(u64),
    ServerMember(u64),
    ServerAdministrator(u64),
    ServerChannel(u64),
    ServerGroup(u64),
    GroupChannel(u64),
}

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone, BorshSchema)]
pub struct CreateGroupInput {
    pub name: [u8; 32],
}

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone, BorshSchema)]
pub struct AddChannelInput {
    pub type_id: u8,
    pub name: [u8; 32],
}

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone, BorshSchema)]
pub struct SetNameInput {
    pub name: [u8; 32],
}

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone, BorshSchema)]
pub struct SetDwellerStatusInput {
    pub status: [u8; 32],
}

/// IPFS hash
#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone, BorshSchema)]
pub struct SetHashInput {
    pub hash: [u8; 64],
}

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone, BorshSchema)]
pub struct InitializeDwellerInput {
    pub name: [u8; 32],
}

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone, BorshSchema)]
pub struct InitializeServerInput {
    pub name: [u8; 32],
}

/// [Instruction::InitializeDweller]
#[allow(clippy::too_many_arguments)]
pub fn initialize_dweller(
    dweller: &Pubkey,
    input: InitializeDwellerInput,
) -> Result<solana_program::instruction::Instruction, ProgramError> {
    let mut data = Instruction::InitializeDweller.try_to_vec()?;
    let mut input = input.try_to_vec()?;
    data.append(&mut input);
    let accounts = vec![AccountMeta::new(*dweller, true)];
    Ok(solana_program::instruction::Instruction {
        program_id: crate::id(),
        accounts,
        data,
    })
}

/// Create [Instruction::CreateDerivedAccount] instruction
pub fn create_derived_account(
    program_id: &Pubkey,
    payer: &Pubkey,
    owner_address: &Pubkey,
    base_program_address: &Pubkey,
    account_to_create: &Pubkey,
    input: AddressTypeInput,
) -> Result<solana_program::instruction::Instruction, ProgramError> {
    let mut data = Instruction::CreateDerivedAccount.try_to_vec()?;
    let mut input = input.try_to_vec()?;
    data.append(&mut input);
    let accounts = vec![
        AccountMeta::new(*payer, true),
        AccountMeta::new_readonly(*owner_address, false),
        AccountMeta::new_readonly(*base_program_address, false),
        AccountMeta::new(*account_to_create, false),
        AccountMeta::new_readonly(sysvar::rent::id(), false),
        AccountMeta::new_readonly(system_program::id(), false),
    ];
    Ok(solana_program::instruction::Instruction {
        program_id: *program_id,
        accounts,
        data,
    })
}

/// [Instruction::InitializeServer]
#[allow(clippy::too_many_arguments)]
pub fn initialize_server(
    dweller_owner: &Pubkey,
    server: &Pubkey,
    dweller_server: &Pubkey,
    server_member: &Pubkey,
    input: InitializeServerInput,
) -> Result<solana_program::instruction::Instruction, ProgramError> {
    let mut data = Instruction::InitializeServer.try_to_vec()?;
    let mut input = input.try_to_vec()?;
    data.append(&mut input);
    let accounts = vec![
        AccountMeta::new(*dweller_owner, true),
        AccountMeta::new(*server, true),
        AccountMeta::new(*dweller_server, false),
        AccountMeta::new(*server_member, false),
    ];

    Ok(solana_program::instruction::Instruction {
        program_id: crate::id(),
        accounts,
        data,
    })
}

/// [Instruction::SetDwellerName]
pub fn set_dweller_name(
    dweller: &Pubkey,
    input: &SetNameInput,
) -> Result<solana_program::instruction::Instruction, ProgramError> {
    let mut data = Instruction::SetDwellerName.try_to_vec()?;
    let mut input = input.try_to_vec()?;
    data.append(&mut input);
    let accounts = vec![AccountMeta::new(*dweller, true)];

    Ok(solana_program::instruction::Instruction {
        program_id: crate::id(),
        accounts,
        data,
    })
}

/// [Instruction::SetDwellerPhoto]
pub fn set_dweller_photo(
    dweller: &Pubkey,
    input: &SetHashInput,
) -> Result<solana_program::instruction::Instruction, ProgramError> {
    let mut data = Instruction::SetDwellerPhoto.try_to_vec()?;
    let mut input = input.try_to_vec()?;
    data.append(&mut input);
    let accounts = vec![AccountMeta::new(*dweller, true)];

    Ok(solana_program::instruction::Instruction {
        program_id: crate::id(),
        accounts,
        data,
    })
}

/// [Instruction::SetDwellerStatus]
pub fn set_dweller_status(
    dweller: &Pubkey,
    input: &SetDwellerStatusInput,
) -> Result<solana_program::instruction::Instruction, ProgramError> {
    let mut data = Instruction::SetDwellerStatus.try_to_vec()?;
    let mut input = input.try_to_vec()?;
    data.append(&mut input);
    let accounts = vec![AccountMeta::new(*dweller, true)];

    Ok(solana_program::instruction::Instruction {
        program_id: crate::id(),
        accounts,
        data,
    })
}

/// [Instruction::AddChannel]
pub fn add_channel(
    dweller: &Pubkey,
    server_administrator: &Pubkey,
    server: &Pubkey,
    server_channel: &Pubkey,
    input: &AddChannelInput,
) -> Result<solana_program::instruction::Instruction, ProgramError> {
    let mut data = Instruction::AddChannel.try_to_vec()?;
    let mut input = input.try_to_vec()?;
    data.append(&mut input);
    let accounts = vec![
        AccountMeta::new(*dweller, true),
        AccountMeta::new_readonly(*server_administrator, false),
        AccountMeta::new(*server, false),
        AccountMeta::new(*server_channel, false),
    ];

    Ok(solana_program::instruction::Instruction {
        program_id: crate::id(),
        accounts,
        data,
    })
}


/// [Instruction::DeleteChannel]
pub fn delete_channel(
dweller: &Pubkey,
server_administrator: &Pubkey,
server: &Pubkey,
server_channel: &Pubkey,
server_channel_last: &Pubkey,
) -> Result<solana_program::instruction::Instruction, ProgramError> {
    let data = Instruction::DeleteChannel.try_to_vec()?;
    let accounts = vec![
        AccountMeta::new(*dweller, true),
        AccountMeta::new_readonly(*server_administrator, false),
        AccountMeta::new(*server, false),
        AccountMeta::new(*server_channel, false),
        AccountMeta::new(*server_channel_last, false),
    ];

    Ok(solana_program::instruction::Instruction {
        program_id: crate::id(),
        accounts,
        data,
    })
}


/// [Instruction::CreateGroup]
pub fn create_group(
    dweller: &Pubkey,
    server_administrator: &Pubkey,
    server: &Pubkey,
    server_group: &Pubkey,
    input:&CreateGroupInput,
    ) -> Result<solana_program::instruction::Instruction, ProgramError> {
        let mut data = Instruction::CreateGroup.try_to_vec()?;
        let mut input = input.try_to_vec()?;
        data.append(&mut input);
        let accounts = vec![
            AccountMeta::new(*dweller, true),
            AccountMeta::new_readonly(*server_administrator, false),
            AccountMeta::new(*server, false),
            AccountMeta::new(*server_group, false),
        ];
    
        Ok(solana_program::instruction::Instruction {
            program_id: crate::id(),
            accounts,
            data,
        })
    }
    

    // /// Accounts:
    // /// - signer    dweller    
    // /// - read      server_administrator
    // /// - write     server
    // /// - write     server_group
    // /// - write     server_group_last
    // /// - write     [group_channel] all channels in group
    // DeleteGroup,

    // /// Accounts:
    // /// - write     server
    // /// - signer    dweller
    // /// - read      server_administrator
    // /// - write     group_channel
    // /// - read      channel
    // AddChannelToGroup,

    // /// Accounts:
    // /// - write     server
    // /// - signer    dweller
    // /// - read      server_administrator
    // /// - read      channel
    // /// - write     group_channel
    // /// - write     group_channel_last
    // RemoveChannelFromGroup,

    // /// Accounts:
    // ///
    // /// - signer    owner of server
    // /// - read      dweller to become admin
    // /// - write     server
    // /// - write     server_administrator
    // AddAdmin,

    // /// Accounts:
    // /// - signer    owner
    // /// - write     server
    // /// - write     admin
    // /// - write     admin_last
    // RemoveAdmin,

    // /// Accounts:
    // ///   - writeable         server     
    // ///   - read signer       dweller
    // ///   - writeable         dweller_server
    // ///   - writeable         server_member
    // ///   - read              server_member_status
    // JoinServer,

    // /// Accounts:
    // ///
    // /// - write, signer     dweller
    // /// - write             server
    // /// - write             server_member
    // /// - write             server_member_last
    // /// - write             dweller_server
    // /// - write             dweller_server_last
    // LeaveServer,

    // /// Accounts:
    // /// - write             server
    // /// - signer            dweller_administrator
    // /// - read, derived     server_administrator
    // /// - read              dweller
    // /// - write             member_status
    // InviteToServer,

    // /// Accounts:
    // /// - read, signer       admin
    // /// - write              server
    // /// - write, derived     member_status
    // /// - write, derived     member_status_last
    // RevokeInviteServer,

    // /// Accounts:
    // /// - signer   admin
    // /// - write    server
    // ///
    // /// Input: [SetNameInput]
    // SetServerName,

    // /// Accounts:
    // /// - signer    admin
    // /// - write     server
    // ///
    // /// Input: [SetHashInput]        
    // SetServerDb,
