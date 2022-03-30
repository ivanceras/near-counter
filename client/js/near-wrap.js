
export function get_account_id() {
    return window.walletConnection.getAccountId();
}

export function contract_get_num() {
    return contract.get_num()
}

export function contract_increment() {
    return contract.increment()
}

export function contract_decrement() {
    return contract.decrement()
}

export function contract_reset() {
    return contract.reset()
}

export function sign_out(){
    walletConnection.signOut();
    window.location.replace(window.location.origin + window.location.pathname);
}

export function request_sign_in(){
    walletConnection.requestSignIn(window.nearConfig.contractName, "Rust Counter Example");
}
