package com.ppaass.protocol.vpn.message;

public enum ProxyMessageBodyType implements MessageBodyType {
    /**
     * TCP data handled.
     */
    TCP_DATA_SUCCESS(0),
    /**
     * TCP connection close.
     */
    TCP_CONNECTION_CLOSE(1),
    /**
     * UDP data handled.
     */
    UDP_DATA_SUCCESS(2),
    /**
     * Connection fail
     */
    TCP_CONNECT_FAIL(3),
    /**
     * Connection success
     */
    TCP_CONNECT_SUCCESS(4),
    /**
     * Fail on transfer TCP data
     */
    TCP_DATA_FAIL(5),
    /**
     * Fail on transfer UDP data
     */
    UDP_DATA_FAIL(6),
    UDP_OVER_TCP_CONNECT_SUCCESS(7),
    UDP_OVER_TCP_CONNECT_FAIL(8);
    private final int value;

    ProxyMessageBodyType(int value) {
        this.value = value;
    }

    @Override
    public int value() {
        return this.value;
    }
}
