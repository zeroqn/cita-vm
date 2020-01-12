pvm.debug('Testing: pvm.debug')
pvm.debug('Test[v]: pvm.debug')

pvm.debug('Testing: pvm.ret')
var buffer_ret = new Buffer([0x54, 0x65, 0x73, 0x74, 0x3a, 0x20, 0x65, 0x6e, 0x76, 0x2e, 0x72, 0x65, 0x74])
pvm.ret(buffer_ret)
pvm.debug('Test[v]: pvm.ret')

pvm.debug('Testing: pvm.save/pvm.load')
var buffer_k = new Buffer([0x65, 0x6e, 0x76, 0x2e, 0x6b])
var buffer_v = new Buffer([0x65, 0x6e, 0x76, 0x2e, 0x76])
pvm.save(buffer_k, buffer_v)

var r = pvm.load(buffer_k)
if (r.subarray(0, 5)[4] == buffer_v[4]) {
    pvm.debug('Test[v]: pvm.save/pvm.load')
} else {
    pvm.debug('Test[x]: pvm.save/pvm.load')
}

pvm.debug('Testing: pvm.address')
var addr = pvm.address()
if (addr[19] == 0x01) {
    pvm.debug('Test[v]: pvm.address')
} else {
    pvm.debug('Test[x]: pvm.address')
}

pvm.debug('Testing: pvm.balance')
var acc1 = new Buffer([
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01]);
var v = pvm.balance(acc1)
if (v[31] == 10) {
    pvm.debug('Test[v]: pvm.balance')
} else {
    pvm.debug('Test[x]: pvm.balance')
}

pvm.debug('Testing: pvm.origin')
var addr = pvm.origin()
if (addr[19] == 0x02) {
    pvm.debug('Test[v]: pvm.origin')
} else {
    pvm.debug('Test[x]: pvm.origin')
}

pvm.debug('Testing: pvm.caller')
var addr = pvm.caller()
if (addr[19] == 0x03) {
    pvm.debug('Test[v]: pvm.caller')
} else {
    pvm.debug('Test[x]: pvm.caller')
}

pvm.debug('Testing: pvm.callvalue')
var callvalue = pvm.callvalue()
if (callvalue[31] == 0x05) {
    pvm.debug('Test[v]: pvm.callvalue')
} else {
    pvm.debug('Test[x]: pvm.callvalue')
}

pvm.debug('Testing: pvm.blockhash')
var blockhash = pvm.blockhash(7)
if (blockhash[31] == 0x07) {
    pvm.debug('Test[v]: pvm.blockhash')
} else {
    pvm.debug('Test[x]: pvm.blockhash')
}

pvm.debug('Testing: pvm.coinbase')
var addr = pvm.coinbase()
if (addr[19] == 0x08) {
    pvm.debug('Test[v]: pvm.coinbase')
} else {
    pvm.debug('Test[x]: pvm.coinbase')
}

pvm.debug('Testing: pvm.timestamp')
var timestamp = pvm.timestamp()
if (timestamp == 0x09) {
    pvm.debug('Test[v]: pvm.timestamp')
} else {
    pvm.debug('Test[x]: pvm.timestamp')
}

pvm.debug('Testing: pvm.number')
var number = pvm.number()
if (number[31] == 0x06) {
    pvm.debug('Test[v]: pvm.number')
} else {
    pvm.debug('Test[x]: pvm.number')
}

pvm.debug('Testing: pvm.difficulty')
var difficulty = pvm.difficulty()
if (difficulty[31] == 0x0a) {
    pvm.debug('Test[v]: pvm.difficulty')
} else {
    pvm.debug('Test[x]: pvm.difficulty')
}

function main(argc, argv) {
    pvm.debug(argc)
    pvm.debug(argv)
    pvm.debug('Main')
}
