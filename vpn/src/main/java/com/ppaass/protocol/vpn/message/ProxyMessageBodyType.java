package com.ppaass.protocol.vpn.message;

public enum ProxyMessageBodyType implements MessageBodyType {
    /**
     * TCP data handled.
     */
    TCP_DATA_SUCCESS(1),
    /**
     * TCP connection close.
     */
    TCP_CONNECTION_CLOSE(2),
    /**
     * UDP data handled.
     */
    UDP_DATA_SUCCESS(3),
    /**
     * Connection fail
     */
    TCP_CONNECT_FAIL(4),
    /**
     * Connection success
     */
    TCP_CONNECT_SUCCESS(5),
    /**
     * Fail on transfer TCP data
     */
    TCP_DATA_FAIL(6),
    /**
     * Fail on transfer UDP data
     */
    UDP_DATA_FAIL(7);
    private final int value;

    ProxyMessageBodyType(int value) {
        this.value = value;
    }

    @Override
    public int value() {
        return this.value;
    }
}
