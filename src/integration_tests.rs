#[cfg(test)]
mod tests {
    use crate::helpers::CwTemplateContract;
    use crate::msg::InstantiateMsg;
    use cosmwasm_std::{Addr, Coin, Empty, Uint128};
    use cw_multi_test::{App, AppBuilder, Contract, ContractWrapper, Executor};

    pub fn contract_template() -> Box<dyn Contract<Empty>> {
        let contract = ContractWrapper::new(
            crate::contract::execute,
            crate::contract::instantiate,
            crate::contract::query,
        );
        Box::new(contract)
    }

    const USER: &str = "USER";
    const ADMIN: &str = "ADMIN";
    const NATIVE_DENOM: &str = "denom";

    fn mock_app() -> App {
        AppBuilder::new().build(|router, _, storage| {
            router
                .bank
                .init_balance(
                    storage,
                    &Addr::unchecked(USER),
                    vec![Coin {
                        denom: NATIVE_DENOM.to_string(),
                        amount: Uint128::new(1),
                    }],
                )
                .unwrap();
        })
    }

    fn proper_instantiate() -> (App, CwTemplateContract) {
        let mut app = mock_app();
        let cw_template_id = app.store_code(contract_template());

        let msg = InstantiateMsg { max_capacity: 5u8 };
        let cw_template_contract_addr = app
            .instantiate_contract(
                cw_template_id,
                Addr::unchecked(USER),
                &msg,
                &[],
                "test",
                None,
            )
            .unwrap();

        let cw_template_contract = CwTemplateContract(cw_template_contract_addr);

        (app, cw_template_contract)
    }

    mod consume {
        use super::*;
        use crate::msg::ExecuteMsg;

        #[test]
        fn consume_percentage() {
            let (mut app, cw_template_contract) = proper_instantiate();

            let msg = ExecuteMsg::Consume { amount: 1 };
            let cosmos_msg = cw_template_contract.call(msg).unwrap();

            app.execute(Addr::unchecked(USER), cosmos_msg).unwrap();
        }

        #[test]
        fn transfer() {
            let (mut app, cw_template_contract) = proper_instantiate();

            let msg = ExecuteMsg::Transfer {
                to: Addr::unchecked(ADMIN),
            };
            let cosmos_msg = cw_template_contract.call(msg).unwrap();

            app.execute(Addr::unchecked(USER), cosmos_msg).unwrap();
        }
    }
}
