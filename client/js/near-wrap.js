
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
