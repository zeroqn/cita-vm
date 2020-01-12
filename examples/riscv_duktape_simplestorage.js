function getInt64Bytes(x) {
    buffer = new Buffer(8)
    buffer[0] = (x / 2 ** 56) & 0xFF
    buffer[1] = (x / 2 ** 48) & 0xFF
    buffer[2] = (x / 2 ** 40) & 0xFF
    buffer[3] = (x / 2 ** 32) & 0xFF
    buffer[4] = (x / 2 ** 24) & 0xFF
    buffer[5] = (x / 2 ** 16) & 0xFF
    buffer[6] = (x / 2 **8) & 0xFF
    buffer[7] = x & 0xFF
    return buffer
}

function main(argc, argv) {
    if (argv[1] == 'set') {
        var k = argv[2]
        var v = getInt64Bytes(parseInt(argv[3], 10))
        pvm.save(k, v)
    }
    if (argv[1] == 'get') {
        var k = argv[2]
        var v = pvm.load(k)
        pvm.ret(v)
    }
}
