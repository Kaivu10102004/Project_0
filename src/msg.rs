use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Uint128};

#[cw_serde]
pub struct InstantiateMsg {
    pub owner: Addr,
}

#[cw_serde]
pub enum ExecuteMsg {
    SetToken {token_address: Addr},
    SetAPR {amount : u64},
    Stake{amount : u64, sender : Addr},
    WithDraw{amount : u64, sender : Addr},
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(u64)]
    Example {},
}
