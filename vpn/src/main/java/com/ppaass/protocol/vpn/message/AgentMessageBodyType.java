package com.ppaass.protocol.vpn.message;

public enum AgentMessageBodyType implements MessageBodyType {
    /**
     * Create a connection
     */
    TCP_CONNECT(0),
    /**
     * Sending a TCP data
     */
    TCP_DATA(1),
    /**
     * Sending a UDP data
     */
    UDP_DATA(2);
    private final int value;

    AgentMessageBodyType(int value) {
        this.value = value;
    }

    @Override
    public int value() {
        return this.value;
    }
}
