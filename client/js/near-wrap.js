
export function getAccountId() {
    return window.walletConnection.getAccountId();
}

export function contractGetNum() {
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
