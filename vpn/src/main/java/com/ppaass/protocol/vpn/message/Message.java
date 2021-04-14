package com.ppaass.protocol.vpn.message;

public abstract class Message<BT extends MessageBodyType, T extends MessageBody<BT>> {
    private final byte[] encryptionToken;
    private final EncryptionType encryptionType;
    private final T body;

    public Message(byte[] encryptionToken, EncryptionType encryptionType,
                   T body) {
        this.encryptionToken = encryptionToken;
        this.encryptionType = encryptionType;
        this.body = body;
    }

    public byte[] getEncryptionToken() {
        return encryptionToken;
    }

    public EncryptionType getEncryptionType() {
        return encryptionType;
    }

    public T getBody() {
        return body;
    }
}
